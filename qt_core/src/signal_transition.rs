/// C++ type: <span style='color: green;'>```QSignalTransition```</span>
#[repr(C)]
pub struct SignalTransition(u8);

impl SignalTransition {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSignalTransition::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QSignalTransition_metaObject(self as *const ::signal_transition::SignalTransition) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSignalTransition::QSignalTransition()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::signal_transition::SignalTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QSignalTransition::QSignalTransition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::state::State) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSignalTransition::QSignalTransition(QState* sourceState = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::object::Object, *const ::libc::c_char)) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSignalTransition::QSignalTransition(const QObject* sender, const char* signal)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::object::Object, *const ::libc::c_char, *mut ::state::State)) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSignalTransition::QSignalTransition(const QObject* sender, const char* signal, QState* sourceState = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition>
    where Args: overloading::SignalTransitionNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QObject* QSignalTransition::senderObject() const```</span>
  ///
  ///
  pub fn sender_object(&self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QSignalTransition_senderObject(self as *const ::signal_transition::SignalTransition) }
  }

  /// C++ method: <span style='color: green;'>```void QSignalTransition::setSenderObject(const QObject* sender)```</span>
  ///
  ///
  pub unsafe fn set_sender_object(&mut self, sender: *const ::object::Object) {
    ::ffi::qt_core_c_QSignalTransition_setSenderObject(self as *mut ::signal_transition::SignalTransition, sender)
  }

  /// C++ method: <span style='color: green;'>```void QSignalTransition::setSignal(const QByteArray& signal)```</span>
  ///
  ///
  pub fn set_signal(&mut self, signal: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QSignalTransition_setSignal(self as *mut ::signal_transition::SignalTransition,
                                                   signal as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QSignalTransition::signal() const```</span>
  ///
  ///
  pub fn signal(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSignalTransition_signal_to_output(self as *const ::signal_transition::SignalTransition,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSignalTransition::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSignalTransition_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSignalTransition::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSignalTransition_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::signal_transition::SignalTransition {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSignalTransition_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SignalTransition`.
  pub struct Signals<'a>(&'a ::signal_transition::SignalTransition);
  /// Represents a built-in Qt signal `QSignalTransition::triggered`.
  ///
  /// An object of this type can be created from `SignalTransition` with `object.signals().triggered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalTransition` object.
  pub struct Triggered<'a>(&'a ::signal_transition::SignalTransition);
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
  /// Represents a built-in Qt signal `QSignalTransition::targetStatesChanged`.
  ///
  /// An object of this type can be created from `SignalTransition` with `object.signals().target_states_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalTransition` object.
  pub struct TargetStatesChanged<'a>(&'a ::signal_transition::SignalTransition);
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
  /// Represents a built-in Qt signal `QSignalTransition::signalChanged`.
  ///
  /// An object of this type can be created from `SignalTransition` with `object.signals().signal_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalTransition` object.
  pub struct SignalChanged<'a>(&'a ::signal_transition::SignalTransition);
  impl<'a> ::connection::Receiver for SignalChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2signalChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for SignalChanged<'a> {}
  /// Represents a built-in Qt signal `QSignalTransition::senderObjectChanged`.
  ///
  /// An object of this type can be created from `SignalTransition` with `object.signals().sender_object_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalTransition` object.
  pub struct SenderObjectChanged<'a>(&'a ::signal_transition::SignalTransition);
  impl<'a> ::connection::Receiver for SenderObjectChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2senderObjectChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for SenderObjectChanged<'a> {}
  /// Represents a built-in Qt signal `QSignalTransition::targetStateChanged`.
  ///
  /// An object of this type can be created from `SignalTransition` with `object.signals().target_state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SignalTransition` object.
  pub struct TargetStateChanged<'a>(&'a ::signal_transition::SignalTransition);
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
    /// Returns an object representing a built-in Qt signal `QSignalTransition::triggered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn triggered(&self) -> Triggered {
      Triggered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSignalTransition::targetStatesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_states_changed(&self) -> TargetStatesChanged {
      TargetStatesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSignalTransition::signalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn signal_changed(&self) -> SignalChanged {
      SignalChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSignalTransition::senderObjectChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sender_object_changed(&self) -> SenderObjectChanged {
      SenderObjectChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSignalTransition::targetStateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn target_state_changed(&self) -> TargetStateChanged {
      TargetStateChanged(self.0)
    }
  }
  impl ::signal_transition::SignalTransition {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::signal_transition::SignalTransition> for ::abstract_transition::AbstractTransition {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::signal_transition::SignalTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QAbstractTransition(self as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::signal_transition::SignalTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QAbstractTransition(self as *const ::abstract_transition::AbstractTransition as *mut ::abstract_transition::AbstractTransition) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::signal_transition::SignalTransition> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::signal_transition::SignalTransition> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::signal_transition::SignalTransition> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_dynamic_cast_QSignalTransition_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_transition::AbstractTransition> for ::signal_transition::SignalTransition {
  fn static_cast_mut(&mut self) -> &mut ::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_static_cast_QAbstractTransition_ptr(self as *mut ::signal_transition::SignalTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_static_cast_QAbstractTransition_ptr(self as *const ::signal_transition::SignalTransition as *mut ::signal_transition::SignalTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::signal_transition::SignalTransition {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QSignalTransition_G_static_cast_QObject_ptr(self as *mut ::signal_transition::SignalTransition)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_static_cast_QObject_ptr(self as *const ::signal_transition::SignalTransition as *mut ::signal_transition::SignalTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::signal_transition::SignalTransition> for ::abstract_transition::AbstractTransition {
unsafe fn static_cast_mut(&mut self) -> &mut ::signal_transition::SignalTransition {
let ffi_result = ::ffi::qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QAbstractTransition(self as *mut ::abstract_transition::AbstractTransition);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::signal_transition::SignalTransition {
let ffi_result = ::ffi::qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QAbstractTransition(self as *const ::abstract_transition::AbstractTransition as *mut ::abstract_transition::AbstractTransition);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::signal_transition::SignalTransition> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::signal_transition::SignalTransition {
    let ffi_result =
      ::ffi::qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::signal_transition::SignalTransition {
    let ffi_result = ::ffi::qt_core_c_QSignalTransition_G_static_cast_QSignalTransition_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::signal_transition::SignalTransition {
  type Target = ::abstract_transition::AbstractTransition;
  fn deref(&self) -> &::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_static_cast_QAbstractTransition_ptr(self as *const ::signal_transition::SignalTransition as *mut ::signal_transition::SignalTransition) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::signal_transition::SignalTransition {
  fn deref_mut(&mut self) -> &mut ::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSignalTransition_G_static_cast_QAbstractTransition_ptr(self as *mut ::signal_transition::SignalTransition) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SignalTransition::new_unsafe](../struct.SignalTransition.html#method.new_unsafe) method.
  pub trait SignalTransitionNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition>;
  }
  impl SignalTransitionNewUnsafeArgs for (*const ::object::Object, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition> {
      let sender = self.0;
      let signal = self.1;
      let ffi_result = ::ffi::qt_core_c_QSignalTransition_new_sender_signal(sender, signal);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SignalTransitionNewUnsafeArgs for (*const ::object::Object, *const ::libc::c_char, *mut ::state::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition> {
      let sender = self.0;
      let signal = self.1;
      let source_state = self.2;
      let ffi_result = ::ffi::qt_core_c_QSignalTransition_new_sender_signal_sourceState(sender, signal, source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SignalTransitionNewUnsafeArgs for *mut ::state::State {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::signal_transition::SignalTransition> {
      let source_state = self;
      let ffi_result = ::ffi::qt_core_c_QSignalTransition_new_sourceState(source_state);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
