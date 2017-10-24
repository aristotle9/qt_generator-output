/// C++ type: <span style='color: green;'>```QAnimationGroup```</span>
#[repr(C)]
pub struct AnimationGroup(u8);

impl AnimationGroup {
  /// C++ method: <span style='color: green;'>```void QAnimationGroup::addAnimation(QAbstractAnimation* animation)```</span>
  ///
  ///
  pub unsafe fn add_animation(&mut self, animation: *mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QAnimationGroup_addAnimation(self as *mut ::animation_group::AnimationGroup, animation)
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QAnimationGroup::animationAt(int index) const```</span>
  ///
  ///
  pub fn animation_at(&self, index: ::libc::c_int) -> *mut ::abstract_animation::AbstractAnimation {
    unsafe { ::ffi::qt_core_c_QAnimationGroup_animationAt(self as *const ::animation_group::AnimationGroup, index) }
  }

  /// C++ method: <span style='color: green;'>```int QAnimationGroup::animationCount() const```</span>
  ///
  ///
  pub fn animation_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAnimationGroup_animationCount(self as *const ::animation_group::AnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```void QAnimationGroup::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QAnimationGroup_clear(self as *mut ::animation_group::AnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```int QAnimationGroup::indexOfAnimation(QAbstractAnimation* animation) const```</span>
  ///
  ///
  pub unsafe fn index_of_animation(&self, animation: *mut ::abstract_animation::AbstractAnimation) -> ::libc::c_int {
    ::ffi::qt_core_c_QAnimationGroup_indexOfAnimation(self as *const ::animation_group::AnimationGroup, animation)
  }

  /// C++ method: <span style='color: green;'>```void QAnimationGroup::insertAnimation(int index, QAbstractAnimation* animation)```</span>
  ///
  ///
  pub unsafe fn insert_animation(&mut self,
                                 index: ::libc::c_int,
                                 animation: *mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QAnimationGroup_insertAnimation(self as *mut ::animation_group::AnimationGroup,
                                                     index,
                                                     animation)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAnimationGroup::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QAnimationGroup_metaObject(self as *const ::animation_group::AnimationGroup) }
  }

  /// C++ method: <span style='color: green;'>```void QAnimationGroup::removeAnimation(QAbstractAnimation* animation)```</span>
  ///
  ///
  pub unsafe fn remove_animation(&mut self, animation: *mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QAnimationGroup_removeAnimation(self as *mut ::animation_group::AnimationGroup, animation)
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QAnimationGroup::takeAnimation(int index)```</span>
  ///
  ///
  pub fn take_animation(&mut self, index: ::libc::c_int) -> *mut ::abstract_animation::AbstractAnimation {
    unsafe { ::ffi::qt_core_c_QAnimationGroup_takeAnimation(self as *mut ::animation_group::AnimationGroup, index) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAnimationGroup::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAnimationGroup_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAnimationGroup::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAnimationGroup_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::animation_group::AnimationGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAnimationGroup_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AnimationGroup`.
  pub struct Signals<'a>(&'a ::animation_group::AnimationGroup);
  /// Represents a built-in Qt signal `QAnimationGroup::stateChanged`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct StateChanged<'a>(&'a ::animation_group::AnimationGroup);
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
  /// Represents a built-in Qt signal `QAnimationGroup::directionChanged`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.signals().direction_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct DirectionChanged<'a>(&'a ::animation_group::AnimationGroup);
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
  /// Represents a built-in Qt signal `QAnimationGroup::currentLoopChanged`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.signals().current_loop_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct CurrentLoopChanged<'a>(&'a ::animation_group::AnimationGroup);
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
  /// Represents a built-in Qt signal `QAnimationGroup::finished`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct Finished<'a>(&'a ::animation_group::AnimationGroup);
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
    /// Returns an object representing a built-in Qt signal `QAnimationGroup::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAnimationGroup::directionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn direction_changed(&self) -> DirectionChanged {
      DirectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAnimationGroup::currentLoopChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_loop_changed(&self) -> CurrentLoopChanged {
      CurrentLoopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAnimationGroup::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AnimationGroup`.
  pub struct Slots<'a>(&'a ::animation_group::AnimationGroup);
  /// Represents a built-in Qt slot `QAnimationGroup::setCurrentTime`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.slots().set_current_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct SetCurrentTime<'a>(&'a ::animation_group::AnimationGroup);
  impl<'a> ::connection::Receiver for SetCurrentTime<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentTime(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QAnimationGroup::resume`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.slots().resume()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct Resume<'a>(&'a ::animation_group::AnimationGroup);
  impl<'a> ::connection::Receiver for Resume<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resume()\0"
    }
  }
  /// Represents a built-in Qt slot `QAnimationGroup::pause`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.slots().pause()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct Pause<'a>(&'a ::animation_group::AnimationGroup);
  impl<'a> ::connection::Receiver for Pause<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1pause()\0"
    }
  }
  /// Represents a built-in Qt slot `QAnimationGroup::setPaused`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.slots().set_paused()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct SetPaused<'a>(&'a ::animation_group::AnimationGroup);
  impl<'a> ::connection::Receiver for SetPaused<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPaused(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QAnimationGroup::start`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct Start<'a>(&'a ::animation_group::AnimationGroup);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = (&'static ::abstract_animation::DeletionPolicy,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start(QAbstractAnimation::DeletionPolicy)\0"
    }
  }
  /// Represents a built-in Qt slot `QAnimationGroup::stop`.
  ///
  /// An object of this type can be created from `AnimationGroup` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationGroup` object.
  pub struct Stop<'a>(&'a ::animation_group::AnimationGroup);
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
    /// Returns an object representing a built-in Qt slot `QAnimationGroup::setCurrentTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_time(&self) -> SetCurrentTime {
      SetCurrentTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAnimationGroup::resume`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resume(&self) -> Resume {
      Resume(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAnimationGroup::pause`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pause(&self) -> Pause {
      Pause(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAnimationGroup::setPaused`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_paused(&self) -> SetPaused {
      SetPaused(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAnimationGroup::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAnimationGroup::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
  }
  impl ::animation_group::AnimationGroup {
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

impl ::cpp_utils::DynamicCast<::animation_group::AnimationGroup> for ::abstract_animation::AbstractAnimation {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::animation_group::AnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::animation_group::AnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::animation_group::AnimationGroup> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::animation_group::AnimationGroup> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::animation_group::AnimationGroup> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_dynamic_cast_QAnimationGroup_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_animation::AbstractAnimation> for ::animation_group::AnimationGroup {
  fn static_cast_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *mut ::animation_group::AnimationGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *const ::animation_group::AnimationGroup as *mut ::animation_group::AnimationGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::animation_group::AnimationGroup {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QObject_ptr(self as *mut ::animation_group::AnimationGroup)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QObject_ptr(self as *const ::animation_group::AnimationGroup as *mut ::animation_group::AnimationGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::animation_group::AnimationGroup> for ::abstract_animation::AbstractAnimation {
  unsafe fn static_cast_mut(&mut self) -> &mut ::animation_group::AnimationGroup {
    let ffi_result = ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QAbstractAnimation(self as *mut ::abstract_animation::AbstractAnimation);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::animation_group::AnimationGroup {
    let ffi_result = ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QAbstractAnimation(self as *const ::abstract_animation::AbstractAnimation as *mut ::abstract_animation::AbstractAnimation);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::animation_group::AnimationGroup> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::animation_group::AnimationGroup {
    let ffi_result =
      ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::animation_group::AnimationGroup {
    let ffi_result = ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAnimationGroup_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::animation_group::AnimationGroup {
  type Target = ::abstract_animation::AbstractAnimation;
  fn deref(&self) -> &::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *const ::animation_group::AnimationGroup as *mut ::animation_group::AnimationGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::animation_group::AnimationGroup {
  fn deref_mut(&mut self) -> &mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationGroup_G_static_cast_QAbstractAnimation_ptr(self as *mut ::animation_group::AnimationGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
