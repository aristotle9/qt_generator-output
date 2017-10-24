/// C++ type: <span style='color: green;'>```Qt3DInput::QInputSequence```</span>
#[repr(C)]
pub struct InputSequence(u8);

impl InputSequence {
  /// C++ method: <span style='color: green;'>```void Qt3DInput::QInputSequence::addSequence(Qt3DInput::QAbstractActionInput* input)```</span>
  ///
  ///
  pub unsafe fn add_sequence(&mut self, input: *mut ::abstract_action_input::AbstractActionInput) {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_addSequence(self as *mut ::input_sequence::InputSequence, input)
  }

  /// C++ method: <span style='color: green;'>```int Qt3DInput::QInputSequence::buttonInterval() const```</span>
  ///
  ///
  pub fn button_interval(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_buttonInterval(self as *const ::input_sequence::InputSequence)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QInputSequence::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_metaObject(self as *const ::input_sequence::InputSequence) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QInputSequence::QInputSequence()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::input_sequence::InputSequence> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QInputSequence::QInputSequence(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::input_sequence::InputSequence> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QInputSequence::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_qt_metacall(self as *mut ::input_sequence::InputSequence,
                                                              arg1 as *const ::qt_core::meta_object::Call,
                                                              arg2,
                                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QInputSequence::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_qt_metacast(self as *mut ::input_sequence::InputSequence, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QInputSequence::removeSequence(Qt3DInput::QAbstractActionInput* input)```</span>
  ///
  ///
  pub unsafe fn remove_sequence(&mut self, input: *mut ::abstract_action_input::AbstractActionInput) {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_removeSequence(self as *mut ::input_sequence::InputSequence, input)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*> Qt3DInput::QInputSequence::sequences() const```</span>
  ///
  ///
  pub fn sequences(&self) -> ::vector::VectorAbstractActionInputMutPtr {
    {
      let mut object: ::vector::VectorAbstractActionInputMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_sequences_to_output(self as *const ::input_sequence::InputSequence, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QInputSequence::setButtonInterval(int buttonInterval)```</span>
  ///
  ///
  pub fn set_button_interval(&mut self, button_interval: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_setButtonInterval(self as *mut ::input_sequence::InputSequence,
                                                                      button_interval)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QInputSequence::setTimeout(int timeout)```</span>
  ///
  ///
  pub fn set_timeout(&mut self, timeout: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_setTimeout(self as *mut ::input_sequence::InputSequence, timeout)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DInput::QInputSequence::timeout() const```</span>
  ///
  ///
  pub fn timeout(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_timeout(self as *const ::input_sequence::InputSequence) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QInputSequence::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QInputSequence::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::input_sequence::InputSequence {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QInputSequence_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `InputSequence`.
  pub struct Signals<'a>(&'a ::input_sequence::InputSequence);
  /// Represents a built-in Qt signal `Qt3DInput::QInputSequence::timeoutChanged`.
  ///
  /// An object of this type can be created from `InputSequence` with `object.signals().timeout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSequence` object.
  pub struct TimeoutChanged<'a>(&'a ::input_sequence::InputSequence);
  impl<'a> ::qt_core::connection::Receiver for TimeoutChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2timeoutChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TimeoutChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QInputSequence::buttonIntervalChanged`.
  ///
  /// An object of this type can be created from `InputSequence` with `object.signals().button_interval_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSequence` object.
  pub struct ButtonIntervalChanged<'a>(&'a ::input_sequence::InputSequence);
  impl<'a> ::qt_core::connection::Receiver for ButtonIntervalChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonIntervalChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonIntervalChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QInputSequence::timeoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn timeout_changed(&self) -> TimeoutChanged {
      TimeoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QInputSequence::buttonIntervalChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn button_interval_changed(&self) -> ButtonIntervalChanged {
      ButtonIntervalChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `InputSequence`.
  pub struct Slots<'a>(&'a ::input_sequence::InputSequence);
  /// Represents a built-in Qt slot `Qt3DInput::QInputSequence::setTimeout`.
  ///
  /// An object of this type can be created from `InputSequence` with `object.slots().set_timeout()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSequence` object.
  pub struct SetTimeout<'a>(&'a ::input_sequence::InputSequence);
  impl<'a> ::qt_core::connection::Receiver for SetTimeout<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTimeout(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QInputSequence::setButtonInterval`.
  ///
  /// An object of this type can be created from `InputSequence` with `object.slots().set_button_interval()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `InputSequence` object.
  pub struct SetButtonInterval<'a>(&'a ::input_sequence::InputSequence);
  impl<'a> ::qt_core::connection::Receiver for SetButtonInterval<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setButtonInterval(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QInputSequence::setTimeout`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_timeout(&self) -> SetTimeout {
      SetTimeout(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QInputSequence::setButtonInterval`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_button_interval(&self) -> SetButtonInterval {
      SetButtonInterval(self.0)
    }
  }
  impl ::input_sequence::InputSequence {
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

impl ::cpp_utils::DynamicCast<::input_sequence::InputSequence> for ::abstract_action_input::AbstractActionInput {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::input_sequence::InputSequence> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_dynamic_cast_Qt3DInput_QInputSequence_ptr(self as *mut ::abstract_action_input::AbstractActionInput) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::input_sequence::InputSequence> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_dynamic_cast_Qt3DInput_QInputSequence_ptr(self as *const ::abstract_action_input::AbstractActionInput as *mut ::abstract_action_input::AbstractActionInput) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::input_sequence::InputSequence {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::input_sequence::InputSequence) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::input_sequence::InputSequence as *mut ::input_sequence::InputSequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_action_input::AbstractActionInput> for ::input_sequence::InputSequence {
  fn static_cast_mut(&mut self) -> &mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *mut ::input_sequence::InputSequence) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *const ::input_sequence::InputSequence as *mut ::input_sequence::InputSequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::input_sequence::InputSequence {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_QObject_ptr(self as *mut ::input_sequence::InputSequence)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_QObject_ptr(self as *const ::input_sequence::InputSequence as *mut ::input_sequence::InputSequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_sequence::InputSequence> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_sequence::InputSequence {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_sequence::InputSequence {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_sequence::InputSequence> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_sequence::InputSequence {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_sequence::InputSequence {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_sequence::InputSequence> for ::abstract_action_input::AbstractActionInput {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_sequence::InputSequence {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_Qt3DInput_QAbstractActionInput(self as *mut ::abstract_action_input::AbstractActionInput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_sequence::InputSequence {
    let ffi_result = ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QInputSequence_ptr_Qt3DInput_QAbstractActionInput(self as *const ::abstract_action_input::AbstractActionInput as *mut ::abstract_action_input::AbstractActionInput);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_sequence::InputSequence {
  type Target = ::abstract_action_input::AbstractActionInput;
  fn deref(&self) -> &::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *const ::input_sequence::InputSequence as *mut ::input_sequence::InputSequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_sequence::InputSequence {
  fn deref_mut(&mut self) -> &mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QInputSequence_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *mut ::input_sequence::InputSequence) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
