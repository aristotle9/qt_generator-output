/// C++ type: <span style='color: green;'>```QEventTransition```</span>
#[repr(C)]
pub struct EventTransition(u8);

impl EventTransition {
  /// C++ method: <span style='color: green;'>```QObject* QEventTransition::eventSource() const```</span>
  ///
  ///
  pub fn event_source(&self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QEventTransition_eventSource(self as *const ::event_transition::EventTransition) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QEventTransition::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QEventTransition_metaObject(self as *const ::event_transition::EventTransition) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QEventTransition::QEventTransition()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::event_transition::EventTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QEventTransition::QEventTransition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::object::Object, &::event::Type)) -> ::cpp_utils::CppBox<::event_transition::EventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEventTransition::QEventTransition(QObject* object, QEvent::Type type)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::object::Object, &::event::Type, *mut ::state::State)) -> ::cpp_utils::CppBox<::event_transition::EventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEventTransition::QEventTransition(QObject* object, QEvent::Type type, QState* sourceState = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::state::State) -> ::cpp_utils::CppBox<::event_transition::EventTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEventTransition::QEventTransition(QState* sourceState = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::event_transition::EventTransition>
    where Args: overloading::EventTransitionNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QEventTransition::setEventSource(QObject* object)```</span>
  ///
  ///
  pub unsafe fn set_event_source(&mut self, object: *mut ::object::Object) {
    ::ffi::qt_core_c_QEventTransition_setEventSource(self as *mut ::event_transition::EventTransition, object)
  }

  /// C++ method: <span style='color: green;'>```void QEventTransition::setEventType(QEvent::Type type)```</span>
  ///
  ///
  pub fn set_event_type(&mut self, type_: &::event::Type) {
    unsafe {
      ::ffi::qt_core_c_QEventTransition_setEventType(self as *mut ::event_transition::EventTransition,
                                                     type_ as *const ::event::Type)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QEventTransition::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QEventTransition_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QEventTransition::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QEventTransition_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::event_transition::EventTransition {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QEventTransition_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `EventTransition`.
  pub struct Signals<'a>(&'a ::event_transition::EventTransition);
  /// Represents a built-in Qt signal `QEventTransition::targetStatesChanged`.
  ///
  /// An object of this type can be created from `EventTransition` with `object.signals().target_states_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EventTransition` object.
  pub struct TargetStatesChanged<'a>(&'a ::event_transition::EventTransition);
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
  /// Represents a built-in Qt signal `QEventTransition::triggered`.
  ///
  /// An object of this type can be created from `EventTransition` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EventTransition` object.
  pub struct Triggered<'a>(&'a ::event_transition::EventTransition);
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
  /// Represents a built-in Qt signal `QEventTransition::targetStateChanged`.
  ///
  /// An object of this type can be created from `EventTransition` with `object.signals().target_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EventTransition` object.
  pub struct TargetStateChanged<'a>(&'a ::event_transition::EventTransition);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QEventTransition::targetStatesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_states_changed(&self) -> TargetStatesChanged {
      TargetStatesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QEventTransition::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QEventTransition::targetStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_state_changed(&self) -> TargetStateChanged {
      TargetStateChanged(self.0)
    }
  }
  impl ::event_transition::EventTransition {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::event_transition::EventTransition> for ::abstract_transition::AbstractTransition {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::event_transition::EventTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QAbstractTransition(self as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::event_transition::EventTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QAbstractTransition(self as *const ::abstract_transition::AbstractTransition as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::event_transition::EventTransition> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::event_transition::EventTransition> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::event_transition::EventTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_dynamic_cast_QEventTransition_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_transition::AbstractTransition> for ::event_transition::EventTransition {
  fn static_cast_mut(&mut self) -> &mut ::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_static_cast_QAbstractTransition_ptr(self as *mut ::event_transition::EventTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_static_cast_QAbstractTransition_ptr(self as *const ::event_transition::EventTransition as *mut ::event_transition::EventTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::event_transition::EventTransition {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QEventTransition_G_static_cast_QObject_ptr(self as *mut ::event_transition::EventTransition)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_static_cast_QObject_ptr(self as *const ::event_transition::EventTransition as *mut ::event_transition::EventTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::event_transition::EventTransition> for ::abstract_transition::AbstractTransition {
unsafe fn static_cast_mut(&mut self) -> &mut ::event_transition::EventTransition {
let ffi_result = ::ffi::qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QAbstractTransition(self as *mut ::abstract_transition::AbstractTransition);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::event_transition::EventTransition {
let ffi_result = ::ffi::qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QAbstractTransition(self as *const ::abstract_transition::AbstractTransition as *mut ::abstract_transition::AbstractTransition);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::event_transition::EventTransition> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::event_transition::EventTransition {
    let ffi_result =
      ::ffi::qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::event_transition::EventTransition {
    let ffi_result = ::ffi::qt_core_c_QEventTransition_G_static_cast_QEventTransition_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::event_transition::EventTransition {
  type Target = ::abstract_transition::AbstractTransition;
  fn deref(&self) -> &::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_static_cast_QAbstractTransition_ptr(self as *const ::event_transition::EventTransition as *mut ::event_transition::EventTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::event_transition::EventTransition {
  fn deref_mut(&mut self) -> &mut ::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventTransition_G_static_cast_QAbstractTransition_ptr(self as *mut ::event_transition::EventTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [EventTransition::new_unsafe](../struct.EventTransition.html#method.new_unsafe) method.
  pub trait EventTransitionNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::event_transition::EventTransition>;
  }
  impl<'a> EventTransitionNewUnsafeArgs for (*mut ::object::Object, &'a ::event::Type) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::event_transition::EventTransition> {
      let object = self.0;
      let type_ = self.1;
      let ffi_result = ::ffi::qt_core_c_QEventTransition_new_object_type(object, type_ as *const ::event::Type);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> EventTransitionNewUnsafeArgs for (*mut ::object::Object, &'a ::event::Type, *mut ::state::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::event_transition::EventTransition> {
      let object = self.0;
      let type_ = self.1;
      let source_state = self.2;
      let ffi_result = ::ffi::qt_core_c_QEventTransition_new_object_type_sourceState(object,
                                                                                     type_ as *const ::event::Type,
                                                                                     source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl EventTransitionNewUnsafeArgs for *mut ::state::State {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::event_transition::EventTransition> {
      let source_state = self;
      let ffi_result = ::ffi::qt_core_c_QEventTransition_new_sourceState(source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
