/// C++ type: <span style='color: green;'>```QAbstractState```</span>
#[repr(C)]
pub struct AbstractState(u8);

impl AbstractState {
  /// C++ method: <span style='color: green;'>```bool QAbstractState::active() const```</span>
  ///
  ///
  pub fn active(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QAbstractState_active(self as *const ::abstract_state::AbstractState) }
  }

  /// C++ method: <span style='color: green;'>```QStateMachine* QAbstractState::machine() const```</span>
  ///
  ///
  pub fn machine(&self) -> *mut ::state_machine::StateMachine {
    unsafe { ::ffi::qt_core_c_QAbstractState_machine(self as *const ::abstract_state::AbstractState) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractState::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QAbstractState_metaObject(self as *const ::abstract_state::AbstractState) }
  }

  /// C++ method: <span style='color: green;'>```QState* QAbstractState::parentState() const```</span>
  ///
  ///
  pub fn parent_state(&self) -> *mut ::state::State {
    unsafe { ::ffi::qt_core_c_QAbstractState_parentState(self as *const ::abstract_state::AbstractState) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractState::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractState_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractState::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractState_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_state::AbstractState {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractState_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractState`.
  pub struct Signals<'a>(&'a ::abstract_state::AbstractState);
  /// Represents a built-in Qt signal `QAbstractState::exited`.
  ///
  /// An object of this type can be created from `AbstractState` with `object.signals().exited()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractState` object.
  pub struct Exited<'a>(&'a ::abstract_state::AbstractState);
  impl<'a> ::connection::Receiver for Exited<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2exited()\0"
    }
  }
  impl<'a> ::connection::Signal for Exited<'a> {}
  /// Represents a built-in Qt signal `QAbstractState::activeChanged`.
  ///
  /// An object of this type can be created from `AbstractState` with `object.signals().active_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractState` object.
  pub struct ActiveChanged<'a>(&'a ::abstract_state::AbstractState);
  impl<'a> ::connection::Receiver for ActiveChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activeChanged(bool)\0"
    }
  }
  impl<'a> ::connection::Signal for ActiveChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractState::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractState` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractState` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_state::AbstractState);
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
  /// Represents a built-in Qt signal `QAbstractState::entered`.
  ///
  /// An object of this type can be created from `AbstractState` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractState` object.
  pub struct Entered<'a>(&'a ::abstract_state::AbstractState);
  impl<'a> ::connection::Receiver for Entered<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2entered()\0"
    }
  }
  impl<'a> ::connection::Signal for Entered<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractState::exited`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exited(&self) -> Exited {
      Exited(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractState::activeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_changed(&self) -> ActiveChanged {
      ActiveChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractState::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractState::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
  }
  impl ::abstract_state::AbstractState {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::abstract_state::AbstractState> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_state::AbstractState> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QAbstractState_G_dynamic_cast_QAbstractState_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_state::AbstractState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractState_G_dynamic_cast_QAbstractState_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::abstract_state::AbstractState {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAbstractState_G_static_cast_QObject_ptr(self as *mut ::abstract_state::AbstractState)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractState_G_static_cast_QObject_ptr(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_state::AbstractState> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result = ::ffi::qt_core_c_QAbstractState_G_static_cast_QAbstractState_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_state::AbstractState {
    let ffi_result = ::ffi::qt_core_c_QAbstractState_G_static_cast_QAbstractState_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_state::AbstractState {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractState_G_static_cast_QObject_ptr(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_state::AbstractState {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAbstractState_G_static_cast_QObject_ptr(self as *mut ::abstract_state::AbstractState)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
