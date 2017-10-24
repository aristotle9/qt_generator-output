/// C++ type: <span style='color: green;'>```QAbstractAnimation```</span>
#[repr(C)]
pub struct AbstractAnimation(u8);

impl AbstractAnimation {
  /// C++ method: <span style='color: green;'>```int QAbstractAnimation::currentLoop() const```</span>
  ///
  ///
  pub fn current_loop(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_currentLoop(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractAnimation::currentLoopTime() const```</span>
  ///
  ///
  pub fn current_loop_time(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QAbstractAnimation_currentLoopTime(self as *const ::abstract_animation::AbstractAnimation)
    }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractAnimation::currentTime() const```</span>
  ///
  ///
  pub fn current_time(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_currentTime(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation::Direction QAbstractAnimation::direction() const```</span>
  ///
  ///
  pub fn direction(&self) -> ::abstract_animation::Direction {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_direction(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractAnimation::duration() const```</span>
  ///
  ///
  pub fn duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_duration(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```QAnimationGroup* QAbstractAnimation::group() const```</span>
  ///
  ///
  pub fn group(&self) -> *mut ::animation_group::AnimationGroup {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_group(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractAnimation::loopCount() const```</span>
  ///
  ///
  pub fn loop_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_loopCount(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractAnimation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_metaObject(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractAnimation::pause()```</span>
  ///
  ///
  pub fn pause(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_pause(self as *mut ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractAnimation::resume()```</span>
  ///
  ///
  pub fn resume(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_resume(self as *mut ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractAnimation::setCurrentTime(int msecs)```</span>
  ///
  ///
  pub fn set_current_time(&mut self, msecs: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QAbstractAnimation_setCurrentTime(self as *mut ::abstract_animation::AbstractAnimation, msecs)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractAnimation::setDirection(QAbstractAnimation::Direction direction)```</span>
  ///
  ///
  pub fn set_direction(&mut self, direction: ::abstract_animation::Direction) {
    unsafe {
      ::ffi::qt_core_c_QAbstractAnimation_setDirection(self as *mut ::abstract_animation::AbstractAnimation,
                                                       direction)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractAnimation::setLoopCount(int loopCount)```</span>
  ///
  ///
  pub fn set_loop_count(&mut self, loop_count: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QAbstractAnimation_setLoopCount(self as *mut ::abstract_animation::AbstractAnimation,
                                                       loop_count)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractAnimation::setPaused(bool arg1)```</span>
  ///
  ///
  pub fn set_paused(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_setPaused(self as *mut ::abstract_animation::AbstractAnimation, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation::start```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QAbstractAnimation::start()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start(&mut self, &::abstract_animation::DeletionPolicy) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QAbstractAnimation::start(QAbstractAnimation::DeletionPolicy policy = ?)```</span>
  ///
  ///
  pub fn start<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::AbstractAnimationStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractAnimation::State QAbstractAnimation::state() const```</span>
  ///
  ///
  pub fn state(&self) -> ::abstract_animation::State {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_state(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QAbstractAnimation::stop()```</span>
  ///
  ///
  pub fn stop(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_stop(self as *mut ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```int QAbstractAnimation::totalDuration() const```</span>
  ///
  ///
  pub fn total_duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAbstractAnimation_totalDuration(self as *const ::abstract_animation::AbstractAnimation) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractAnimation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractAnimation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractAnimation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractAnimation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_animation::AbstractAnimation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractAnimation_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractAnimation`.
  pub struct Signals<'a>(&'a ::abstract_animation::AbstractAnimation);
  /// Represents a built-in Qt signal `QAbstractAnimation::currentLoopChanged`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.signals().current_loop_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct CurrentLoopChanged<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for CurrentLoopChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentLoopChanged(int)\0"
    }
  }
  impl<'a> ::connection::Signal for CurrentLoopChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractAnimation::stateChanged`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct StateChanged<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for StateChanged<'a> {
    type Arguments = (&'static ::abstract_animation::State, &'static ::abstract_animation::State);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stateChanged(QAbstractAnimation::State,QAbstractAnimation::State)\0"
    }
  }
  impl<'a> ::connection::Signal for StateChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractAnimation::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_animation::AbstractAnimation);
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
  /// Represents a built-in Qt signal `QAbstractAnimation::directionChanged`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.signals().direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct DirectionChanged<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for DirectionChanged<'a> {
    type Arguments = (&'static ::abstract_animation::Direction,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2directionChanged(QAbstractAnimation::Direction)\0"
    }
  }
  impl<'a> ::connection::Signal for DirectionChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractAnimation::finished`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct Finished<'a>(&'a ::abstract_animation::AbstractAnimation);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractAnimation::currentLoopChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_loop_changed(&self) -> CurrentLoopChanged {
      CurrentLoopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractAnimation::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractAnimation::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractAnimation::directionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn direction_changed(&self) -> DirectionChanged {
      DirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractAnimation::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractAnimation`.
  pub struct Slots<'a>(&'a ::abstract_animation::AbstractAnimation);
  /// Represents a built-in Qt slot `QAbstractAnimation::setPaused`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.slots().set_paused()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct SetPaused<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for SetPaused<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPaused(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractAnimation::stop`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct Stop<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for Stop<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stop()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractAnimation::setCurrentTime`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.slots().set_current_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct SetCurrentTime<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for SetCurrentTime<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentTime(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractAnimation::start`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct Start<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = (&'static ::abstract_animation::DeletionPolicy,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start(QAbstractAnimation::DeletionPolicy)\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractAnimation::resume`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.slots().resume()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct Resume<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for Resume<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resume()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractAnimation::pause`.
  ///
  /// An object of this type can be created from `AbstractAnimation` with `object.slots().pause()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAnimation` object.
  pub struct Pause<'a>(&'a ::abstract_animation::AbstractAnimation);
  impl<'a> ::connection::Receiver for Pause<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1pause()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAbstractAnimation::setPaused`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_paused(&self) -> SetPaused {
      SetPaused(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractAnimation::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractAnimation::setCurrentTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_time(&self) -> SetCurrentTime {
      SetCurrentTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractAnimation::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractAnimation::resume`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resume(&self) -> Resume {
      Resume(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractAnimation::pause`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pause(&self) -> Pause {
      Pause(self.0)
    }
  }
  impl ::abstract_animation::AbstractAnimation {
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

/// C++ type: <span style='color: green;'>```QAbstractAnimation::DeletionPolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DeletionPolicy {
  /// C++ enum variant: <span style='color: green;'>```KeepWhenStopped = 0```</span>
  Keep = 0,
  /// C++ enum variant: <span style='color: green;'>```DeleteWhenStopped = 1```</span>
  Delete = 1,
}

/// C++ type: <span style='color: green;'>```QAbstractAnimation::Direction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Direction {
  /// C++ enum variant: <span style='color: green;'>```Forward = 0```</span>
  Forward = 0,
  /// C++ enum variant: <span style='color: green;'>```Backward = 1```</span>
  Backward = 1,
}

/// C++ type: <span style='color: green;'>```QAbstractAnimation::State```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum State {
  /// C++ enum variant: <span style='color: green;'>```Stopped = 0```</span>
  Stopped = 0,
  /// C++ enum variant: <span style='color: green;'>```Paused = 1```</span>
  Paused = 1,
  /// C++ enum variant: <span style='color: green;'>```Running = 2```</span>
  Running = 2,
}

impl ::cpp_utils::DynamicCast<::abstract_animation::AbstractAnimation> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_animation::AbstractAnimation> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAbstractAnimation_G_dynamic_cast_QAbstractAnimation_ptr(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_animation::AbstractAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractAnimation_G_dynamic_cast_QAbstractAnimation_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::abstract_animation::AbstractAnimation {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractAnimation_G_static_cast_QObject_ptr(self as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractAnimation_G_static_cast_QObject_ptr(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_animation::AbstractAnimation> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      ::ffi::qt_core_c_QAbstractAnimation_G_static_cast_QAbstractAnimation_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = ::ffi::qt_core_c_QAbstractAnimation_G_static_cast_QAbstractAnimation_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_animation::AbstractAnimation {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractAnimation_G_static_cast_QObject_ptr(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_animation::AbstractAnimation {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractAnimation_G_static_cast_QObject_ptr(self as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractAnimation::start](../struct.AbstractAnimation.html#method.start) method.
  pub trait AbstractAnimationStartArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_animation::AbstractAnimation) -> ();
  }
  impl<'largs> AbstractAnimationStartArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::abstract_animation::AbstractAnimation) -> () {

      unsafe {
        ::ffi::qt_core_c_QAbstractAnimation_start_no_args(original_self as *mut ::abstract_animation::AbstractAnimation)
      }
    }
  }
  impl<'largs> AbstractAnimationStartArgs<'largs> for &'largs ::abstract_animation::DeletionPolicy {
    fn exec(self, original_self: &'largs mut ::abstract_animation::AbstractAnimation) -> () {
      let policy = self;
      unsafe {
        ::ffi::qt_core_c_QAbstractAnimation_start_policy(original_self as *mut ::abstract_animation::AbstractAnimation, policy as *const ::abstract_animation::DeletionPolicy)
      }
    }
  }
}
