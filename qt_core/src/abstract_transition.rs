/// C++ type: <span style='color: green;'>```QAbstractTransition```</span>
#[repr(C)]
pub struct AbstractTransition(u8);

impl AbstractTransition {
  /// C++ method: <span style='color: green;'>```void QAbstractTransition::addAnimation(QAbstractAnimation* animation)```</span>
  ///
  ///
  pub unsafe fn add_animation(&mut self, animation: *mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QAbstractTransition_addAnimation(self as *mut ::abstract_transition::AbstractTransition,
                                                      animation)
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*> QAbstractTransition::animations() const```</span>
  ///
  ///
  pub fn animations(&self) -> ::list::ListAbstractAnimationMutPtr {
    {
      let mut object: ::list::ListAbstractAnimationMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractTransition_animations_to_output(self as *const ::abstract_transition::AbstractTransition, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStateMachine* QAbstractTransition::machine() const```</span>
  ///
  ///
  pub fn machine(&self) -> *mut ::state_machine::StateMachine {
    unsafe { ::ffi::qt_core_c_QAbstractTransition_machine(self as *const ::abstract_transition::AbstractTransition) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractTransition::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QAbstractTransition_metaObject(self as *const ::abstract_transition::AbstractTransition) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTransition::removeAnimation(QAbstractAnimation* animation)```</span>
  ///
  ///
  pub unsafe fn remove_animation(&mut self, animation: *mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QAbstractTransition_removeAnimation(self as *mut ::abstract_transition::AbstractTransition,
                                                         animation)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTransition::setTargetState(QAbstractState* target)```</span>
  ///
  ///
  pub unsafe fn set_target_state(&mut self, target: *mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QAbstractTransition_setTargetState(self as *mut ::abstract_transition::AbstractTransition,
                                                        target)
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTransition::setTargetStates(const QList<QAbstractState*>& targets)```</span>
  ///
  ///
  pub fn set_target_states(&mut self, targets: &::list::ListAbstractStateMutPtr) {
    unsafe {
      ::ffi::qt_core_c_QAbstractTransition_setTargetStates(self as *mut ::abstract_transition::AbstractTransition,
                                                           targets as *const ::list::ListAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractTransition::setTransitionType(QAbstractTransition::TransitionType type)```</span>
  ///
  ///
  pub fn set_transition_type(&mut self, type_: ::abstract_transition::TransitionType) {
    unsafe {
      ::ffi::qt_core_c_QAbstractTransition_setTransitionType(self as *mut ::abstract_transition::AbstractTransition,
                                                             type_)
    }
  }

  /// C++ method: <span style='color: green;'>```QState* QAbstractTransition::sourceState() const```</span>
  ///
  ///
  pub fn source_state(&self) -> *mut ::state::State {
    unsafe {
      ::ffi::qt_core_c_QAbstractTransition_sourceState(self as *const ::abstract_transition::AbstractTransition)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* QAbstractTransition::targetState() const```</span>
  ///
  ///
  pub fn target_state(&self) -> *mut ::abstract_state::AbstractState {
    unsafe {
      ::ffi::qt_core_c_QAbstractTransition_targetState(self as *const ::abstract_transition::AbstractTransition)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*> QAbstractTransition::targetStates() const```</span>
  ///
  ///
  pub fn target_states(&self) -> ::list::ListAbstractStateMutPtr {
    {
      let mut object: ::list::ListAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractTransition_targetStates_to_output(self as *const ::abstract_transition::AbstractTransition, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractTransition::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractTransition_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractTransition::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractTransition_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition::TransitionType QAbstractTransition::transitionType() const```</span>
  ///
  ///
  pub fn transition_type(&self) -> ::abstract_transition::TransitionType {
    unsafe {
      ::ffi::qt_core_c_QAbstractTransition_transitionType(self as *const ::abstract_transition::AbstractTransition)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_transition::AbstractTransition {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractTransition_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractTransition`.
  pub struct Signals<'a>(&'a ::abstract_transition::AbstractTransition);
  /// Represents a built-in Qt signal `QAbstractTransition::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractTransition` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTransition` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_transition::AbstractTransition);
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
  /// Represents a built-in Qt signal `QAbstractTransition::triggered`.
  ///
  /// An object of this type can be created from `AbstractTransition` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTransition` object.
  pub struct Triggered<'a>(&'a ::abstract_transition::AbstractTransition);
  impl<'a> ::connection::Receiver for Triggered<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2triggered()\0"
    }
  }
  impl<'a> ::connection::Signal for Triggered<'a> {}
  /// Represents a built-in Qt signal `QAbstractTransition::targetStateChanged`.
  ///
  /// An object of this type can be created from `AbstractTransition` with `object.signals().target_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTransition` object.
  pub struct TargetStateChanged<'a>(&'a ::abstract_transition::AbstractTransition);
  impl<'a> ::connection::Receiver for TargetStateChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2targetStateChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for TargetStateChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractTransition::targetStatesChanged`.
  ///
  /// An object of this type can be created from `AbstractTransition` with `object.signals().target_states_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTransition` object.
  pub struct TargetStatesChanged<'a>(&'a ::abstract_transition::AbstractTransition);
  impl<'a> ::connection::Receiver for TargetStatesChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2targetStatesChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for TargetStatesChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractTransition::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTransition::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTransition::targetStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_state_changed(&self) -> TargetStateChanged {
      TargetStateChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractTransition::targetStatesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_states_changed(&self) -> TargetStatesChanged {
      TargetStatesChanged(self.0)
    }
  }
  impl ::abstract_transition::AbstractTransition {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QAbstractTransition::TransitionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TransitionType {
  /// C++ enum variant: <span style='color: green;'>```ExternalTransition = 0```</span>
  External = 0,
  /// C++ enum variant: <span style='color: green;'>```InternalTransition = 1```</span>
  Internal = 1,
}

impl ::cpp_utils::DynamicCast<::abstract_transition::AbstractTransition> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_transition::AbstractTransition> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAbstractTransition_G_dynamic_cast_QAbstractTransition_ptr(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_transition::AbstractTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTransition_G_dynamic_cast_QAbstractTransition_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::abstract_transition::AbstractTransition {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTransition_G_static_cast_QObject_ptr(self as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTransition_G_static_cast_QObject_ptr(self as *const ::abstract_transition::AbstractTransition as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_transition::AbstractTransition> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      ::ffi::qt_core_c_QAbstractTransition_G_static_cast_QAbstractTransition_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_transition::AbstractTransition {
    let ffi_result = ::ffi::qt_core_c_QAbstractTransition_G_static_cast_QAbstractTransition_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_transition::AbstractTransition {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTransition_G_static_cast_QObject_ptr(self as *const ::abstract_transition::AbstractTransition as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_transition::AbstractTransition {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractTransition_G_static_cast_QObject_ptr(self as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
