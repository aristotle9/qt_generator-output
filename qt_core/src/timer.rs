/// C++ type: <span style='color: green;'>```QTimer```</span>
#[repr(C)]
pub struct Timer(u8);

impl Timer {
  /// C++ method: <span style='color: green;'>```int QTimer::interval() const```</span>
  ///
  ///
  pub fn interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimer_interval(self as *const ::timer::Timer) }
  }

  /// C++ method: <span style='color: green;'>```bool QTimer::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTimer_isActive(self as *const ::timer::Timer) }
  }

  /// C++ method: <span style='color: green;'>```bool QTimer::isSingleShot() const```</span>
  ///
  ///
  pub fn is_single_shot(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTimer_isSingleShot(self as *const ::timer::Timer) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTimer::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QTimer_metaObject(self as *const ::timer::Timer) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTimer::QTimer()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::timer::Timer> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimer_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTimer::QTimer(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::timer::Timer> {
    let ffi_result = ::ffi::qt_core_c_QTimer_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```int QTimer::remainingTime() const```</span>
  ///
  ///
  pub fn remaining_time(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimer_remainingTime(self as *const ::timer::Timer) }
  }

  /// C++ method: <span style='color: green;'>```void QTimer::setInterval(int msec)```</span>
  ///
  ///
  pub fn set_interval(&mut self, msec: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimer_setInterval(self as *mut ::timer::Timer, msec) }
  }

  /// C++ method: <span style='color: green;'>```void QTimer::setSingleShot(bool singleShot)```</span>
  ///
  ///
  pub fn set_single_shot(&mut self, single_shot: bool) {
    unsafe { ::ffi::qt_core_c_QTimer_setSingleShot(self as *mut ::timer::Timer, single_shot) }
  }

  /// C++ method: <span style='color: green;'>```void QTimer::setTimerType(Qt::TimerType atype)```</span>
  ///
  ///
  pub fn set_timer_type(&mut self, atype: &::qt::TimerType) {
    unsafe { ::ffi::qt_core_c_QTimer_setTimerType(self as *mut ::timer::Timer, atype as *const ::qt::TimerType) }
  }

  /// C++ method: <span style='color: green;'>```QTimer::singleShot```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn single_shot((::libc::c_int, &::qt::TimerType, *const ::object::Object, *const ::libc::c_char)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QTimer::singleShot(int msec, Qt::TimerType timerType, const QObject* receiver, const char* member)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn single_shot((::libc::c_int, *const ::object::Object, *const ::libc::c_char)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QTimer::singleShot(int msec, const QObject* receiver, const char* member)```</span>
  ///
  ///
  pub unsafe fn single_shot<Args>(args: Args) -> ()
    where Args: overloading::TimerSingleShotArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTimer::start```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTimer::start()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTimer::start(int msec)```</span>
  ///
  ///
  pub fn start<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TimerStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QTimer::stop()```</span>
  ///
  ///
  pub fn stop(&mut self) {
    unsafe { ::ffi::qt_core_c_QTimer_stop(self as *mut ::timer::Timer) }
  }

  /// C++ method: <span style='color: green;'>```int QTimer::timerId() const```</span>
  ///
  ///
  pub fn timer_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimer_timerId(self as *const ::timer::Timer) }
  }

  /// C++ method: <span style='color: green;'>```static QString QTimer::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTimer_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTimer::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTimer_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::timer::Timer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTimer_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Timer`.
  pub struct Signals<'a>(&'a ::timer::Timer);
  /// Represents a built-in Qt signal `QTimer::timeout`.
  ///
  /// An object of this type can be created from `Timer` with `object.signals().timeout()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Timer` object.
  pub struct Timeout<'a>(&'a ::timer::Timer);
  impl<'a> ::connection::Receiver for Timeout<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2timeout()\0"
    }
  }
  impl<'a> ::connection::Signal for Timeout<'a> {}
  /// Represents a built-in Qt signal `QTimer::objectNameChanged`.
  ///
  /// An object of this type can be created from `Timer` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Timer` object.
  pub struct ObjectNameChanged<'a>(&'a ::timer::Timer);
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
    /// Returns an object representing a built-in Qt signal `QTimer::timeout`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn timeout(&self) -> Timeout {
      Timeout(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimer::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Timer`.
  pub struct Slots<'a>(&'a ::timer::Timer);
  /// Represents a built-in Qt slot `QTimer::start`.
  ///
  /// An object of this type can be created from `Timer` with `object.slots().start_c_int()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Timer` object.
  pub struct StartCInt<'a>(&'a ::timer::Timer);
  impl<'a> ::connection::Receiver for StartCInt<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTimer::start`.
  ///
  /// An object of this type can be created from `Timer` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Timer` object.
  pub struct Start<'a>(&'a ::timer::Timer);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start()\0"
    }
  }
  /// Represents a built-in Qt slot `QTimer::stop`.
  ///
  /// An object of this type can be created from `Timer` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Timer` object.
  pub struct Stop<'a>(&'a ::timer::Timer);
  impl<'a> ::connection::Receiver for Stop<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stop()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTimer::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start_c_int(&self) -> StartCInt {
      StartCInt(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimer::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimer::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
  }
  impl ::timer::Timer {
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

impl ::cpp_utils::DynamicCast<::timer::Timer> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::timer::Timer> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimer_G_dynamic_cast_QTimer_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::timer::Timer> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTimer_G_dynamic_cast_QTimer_ptr(self as *const ::object::Object as *mut ::object::Object)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::timer::Timer {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimer_G_static_cast_QObject_ptr(self as *mut ::timer::Timer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTimer_G_static_cast_QObject_ptr(self as *const ::timer::Timer as *mut ::timer::Timer)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::timer::Timer> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::timer::Timer {
    let ffi_result = ::ffi::qt_core_c_QTimer_G_static_cast_QTimer_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::timer::Timer {
    let ffi_result =
      ::ffi::qt_core_c_QTimer_G_static_cast_QTimer_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::timer::Timer {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTimer_G_static_cast_QObject_ptr(self as *const ::timer::Timer as *mut ::timer::Timer)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::timer::Timer {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimer_G_static_cast_QObject_ptr(self as *mut ::timer::Timer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Timer::single_shot](../struct.Timer.html#method.single_shot) method.
  pub trait TimerSingleShotArgs {
    unsafe fn exec(self) -> ();
  }
  impl TimerSingleShotArgs for (::libc::c_int, *const ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self) -> () {
      let msec = self.0;
      let receiver = self.1;
      let member = self.2;
      ::ffi::qt_core_c_QTimer_singleShot_msec_receiver_member(msec, receiver, member)
    }
  }
  impl<'a> TimerSingleShotArgs for (::libc::c_int, &'a ::qt::TimerType, *const ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self) -> () {
      let msec = self.0;
      let timer_type = self.1;
      let receiver = self.2;
      let member = self.3;
      ::ffi::qt_core_c_QTimer_singleShot_msec_timerType_receiver_member(msec,
                                                                        timer_type as *const ::qt::TimerType,
                                                                        receiver,
                                                                        member)
    }
  }
  /// This trait represents a set of arguments accepted by [Timer::start](../struct.Timer.html#method.start) method.
  pub trait TimerStartArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::timer::Timer) -> ();
  }
  impl<'largs> TimerStartArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::timer::Timer) -> () {
      let msec = self;
      unsafe { ::ffi::qt_core_c_QTimer_start_msec(original_self as *mut ::timer::Timer, msec) }
    }
  }
  impl<'largs> TimerStartArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::timer::Timer) -> () {

      unsafe { ::ffi::qt_core_c_QTimer_start_no_args(original_self as *mut ::timer::Timer) }
    }
  }
}
