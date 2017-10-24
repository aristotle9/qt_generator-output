/// C++ type: <span style='color: green;'>```QFinalState```</span>
#[repr(C)]
pub struct FinalState(u8);

impl FinalState {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFinalState::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QFinalState_metaObject(self as *const ::final_state::FinalState) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFinalState::QFinalState()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::final_state::FinalState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFinalState_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFinalState::QFinalState(QState* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::state::State) -> ::cpp_utils::CppBox<::final_state::FinalState> {
    let ffi_result = ::ffi::qt_core_c_QFinalState_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```static QString QFinalState::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFinalState_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFinalState::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFinalState_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::final_state::FinalState {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QFinalState_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FinalState`.
  pub struct Signals<'a>(&'a ::final_state::FinalState);
  /// Represents a built-in Qt signal `QFinalState::activeChanged`.
  ///
  /// An object of this type can be created from `FinalState` with `object.signals().active_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FinalState` object.
  pub struct ActiveChanged<'a>(&'a ::final_state::FinalState);
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
  /// Represents a built-in Qt signal `QFinalState::entered`.
  ///
  /// An object of this type can be created from `FinalState` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FinalState` object.
  pub struct Entered<'a>(&'a ::final_state::FinalState);
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
  /// Represents a built-in Qt signal `QFinalState::exited`.
  ///
  /// An object of this type can be created from `FinalState` with `object.signals().exited()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FinalState` object.
  pub struct Exited<'a>(&'a ::final_state::FinalState);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFinalState::activeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_changed(&self) -> ActiveChanged {
      ActiveChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFinalState::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFinalState::exited`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exited(&self) -> Exited {
      Exited(self.0)
    }
  }
  impl ::final_state::FinalState {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::final_state::FinalState> for ::abstract_state::AbstractState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::final_state::FinalState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::final_state::FinalState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::final_state::FinalState> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::final_state::FinalState> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::final_state::FinalState> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFinalState_G_dynamic_cast_QFinalState_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_state::AbstractState> for ::final_state::FinalState {
  fn static_cast_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFinalState_G_static_cast_QAbstractState_ptr(self as *mut ::final_state::FinalState) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_state::AbstractState {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFinalState_G_static_cast_QAbstractState_ptr(self as *const ::final_state::FinalState as *mut ::final_state::FinalState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::final_state::FinalState {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFinalState_G_static_cast_QObject_ptr(self as *mut ::final_state::FinalState) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFinalState_G_static_cast_QObject_ptr(self as *const ::final_state::FinalState as *mut ::final_state::FinalState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::final_state::FinalState> for ::abstract_state::AbstractState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::final_state::FinalState {
    let ffi_result = ::ffi::qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QAbstractState(self as *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::final_state::FinalState {
    let ffi_result = ::ffi::qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QAbstractState(self as *const ::abstract_state::AbstractState as *mut ::abstract_state::AbstractState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::final_state::FinalState> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::final_state::FinalState {
    let ffi_result =
      ::ffi::qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::final_state::FinalState {
    let ffi_result = ::ffi::qt_core_c_QFinalState_G_static_cast_QFinalState_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::final_state::FinalState {
  type Target = ::abstract_state::AbstractState;
  fn deref(&self) -> &::abstract_state::AbstractState {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFinalState_G_static_cast_QAbstractState_ptr(self as *const ::final_state::FinalState as *mut ::final_state::FinalState) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::final_state::FinalState {
  fn deref_mut(&mut self) -> &mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFinalState_G_static_cast_QAbstractState_ptr(self as *mut ::final_state::FinalState) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
