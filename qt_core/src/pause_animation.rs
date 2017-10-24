/// C++ type: <span style='color: green;'>```QPauseAnimation```</span>
#[repr(C)]
pub struct PauseAnimation(u8);

impl PauseAnimation {
  /// C++ method: <span style='color: green;'>```virtual int QPauseAnimation::duration() const```</span>
  ///
  ///
  pub fn duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QPauseAnimation_duration(self as *const ::pause_animation::PauseAnimation) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPauseAnimation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QPauseAnimation_metaObject(self as *const ::pause_animation::PauseAnimation) }
  }

  /// C++ method: <span style='color: green;'>```QPauseAnimation::QPauseAnimation```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPauseAnimation::QPauseAnimation()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPauseAnimation::QPauseAnimation(int msecs)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>
    where Args: overloading::PauseAnimationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPauseAnimation::QPauseAnimation```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPauseAnimation::QPauseAnimation(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_int, *mut ::object::Object)) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPauseAnimation::QPauseAnimation(int msecs, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>
    where Args: overloading::PauseAnimationNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPauseAnimation::setDuration(int msecs)```</span>
  ///
  ///
  pub fn set_duration(&mut self, msecs: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QPauseAnimation_setDuration(self as *mut ::pause_animation::PauseAnimation, msecs) }
  }

  /// C++ method: <span style='color: green;'>```static QString QPauseAnimation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QPauseAnimation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPauseAnimation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QPauseAnimation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::pause_animation::PauseAnimation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QPauseAnimation_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PauseAnimation`.
  pub struct Signals<'a>(&'a ::pause_animation::PauseAnimation);
  /// Represents a built-in Qt signal `QPauseAnimation::currentLoopChanged`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.signals().current_loop_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct CurrentLoopChanged<'a>(&'a ::pause_animation::PauseAnimation);
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
  /// Represents a built-in Qt signal `QPauseAnimation::finished`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct Finished<'a>(&'a ::pause_animation::PauseAnimation);
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
  /// Represents a built-in Qt signal `QPauseAnimation::stateChanged`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct StateChanged<'a>(&'a ::pause_animation::PauseAnimation);
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
  /// Represents a built-in Qt signal `QPauseAnimation::directionChanged`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.signals().direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct DirectionChanged<'a>(&'a ::pause_animation::PauseAnimation);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPauseAnimation::currentLoopChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_loop_changed(&self) -> CurrentLoopChanged {
      CurrentLoopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPauseAnimation::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPauseAnimation::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPauseAnimation::directionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn direction_changed(&self) -> DirectionChanged {
      DirectionChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PauseAnimation`.
  pub struct Slots<'a>(&'a ::pause_animation::PauseAnimation);
  /// Represents a built-in Qt slot `QPauseAnimation::stop`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct Stop<'a>(&'a ::pause_animation::PauseAnimation);
  impl<'a> ::connection::Receiver for Stop<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stop()\0"
    }
  }
  /// Represents a built-in Qt slot `QPauseAnimation::setPaused`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.slots().set_paused()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct SetPaused<'a>(&'a ::pause_animation::PauseAnimation);
  impl<'a> ::connection::Receiver for SetPaused<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPaused(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QPauseAnimation::resume`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.slots().resume()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct Resume<'a>(&'a ::pause_animation::PauseAnimation);
  impl<'a> ::connection::Receiver for Resume<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resume()\0"
    }
  }
  /// Represents a built-in Qt slot `QPauseAnimation::setCurrentTime`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.slots().set_current_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct SetCurrentTime<'a>(&'a ::pause_animation::PauseAnimation);
  impl<'a> ::connection::Receiver for SetCurrentTime<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentTime(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPauseAnimation::pause`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.slots().pause()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct Pause<'a>(&'a ::pause_animation::PauseAnimation);
  impl<'a> ::connection::Receiver for Pause<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1pause()\0"
    }
  }
  /// Represents a built-in Qt slot `QPauseAnimation::start`.
  ///
  /// An object of this type can be created from `PauseAnimation` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PauseAnimation` object.
  pub struct Start<'a>(&'a ::pause_animation::PauseAnimation);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = (&'static ::abstract_animation::DeletionPolicy,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start(QAbstractAnimation::DeletionPolicy)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QPauseAnimation::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPauseAnimation::setPaused`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_paused(&self) -> SetPaused {
      SetPaused(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPauseAnimation::resume`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resume(&self) -> Resume {
      Resume(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPauseAnimation::setCurrentTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_time(&self) -> SetCurrentTime {
      SetCurrentTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPauseAnimation::pause`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pause(&self) -> Pause {
      Pause(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPauseAnimation::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
  }
  impl ::pause_animation::PauseAnimation {
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

impl ::cpp_utils::DynamicCast<::pause_animation::PauseAnimation> for ::abstract_animation::AbstractAnimation {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pause_animation::PauseAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_dynamic_cast_QPauseAnimation_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pause_animation::PauseAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_dynamic_cast_QPauseAnimation_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::pause_animation::PauseAnimation> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::pause_animation::PauseAnimation> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QPauseAnimation_G_dynamic_cast_QPauseAnimation_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::pause_animation::PauseAnimation> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_dynamic_cast_QPauseAnimation_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_animation::AbstractAnimation> for ::pause_animation::PauseAnimation {
  fn static_cast_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QAbstractAnimation_ptr(self as *mut ::pause_animation::PauseAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QAbstractAnimation_ptr(self as *const ::pause_animation::PauseAnimation as *mut ::pause_animation::PauseAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::pause_animation::PauseAnimation {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QObject_ptr(self as *mut ::pause_animation::PauseAnimation)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QObject_ptr(self as *const ::pause_animation::PauseAnimation as *mut ::pause_animation::PauseAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pause_animation::PauseAnimation> for ::abstract_animation::AbstractAnimation {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pause_animation::PauseAnimation {
    let ffi_result = ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QPauseAnimation_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pause_animation::PauseAnimation {
    let ffi_result = ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QPauseAnimation_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::pause_animation::PauseAnimation> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::pause_animation::PauseAnimation {
    let ffi_result =
      ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QPauseAnimation_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::pause_animation::PauseAnimation {
    let ffi_result = ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QPauseAnimation_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::pause_animation::PauseAnimation {
  type Target = ::abstract_animation::AbstractAnimation;
  fn deref(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QAbstractAnimation_ptr(self as *const ::pause_animation::PauseAnimation as *mut ::pause_animation::PauseAnimation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::pause_animation::PauseAnimation {
  fn deref_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_G_static_cast_QAbstractAnimation_ptr(self as *mut ::pause_animation::PauseAnimation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PauseAnimation::new](../struct.PauseAnimation.html#method.new) method.
  pub trait PauseAnimationNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>;
  }
  impl PauseAnimationNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation> {
      let msecs = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_new_msecs(msecs) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PauseAnimationNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QPauseAnimation_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [PauseAnimation::new_unsafe](../struct.PauseAnimation.html#method.new_unsafe) method.
  pub trait PauseAnimationNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation>;
  }
  impl PauseAnimationNewUnsafeArgs for (::libc::c_int, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation> {
      let msecs = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QPauseAnimation_new_msecs_parent(msecs, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl PauseAnimationNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::pause_animation::PauseAnimation> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QPauseAnimation_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
