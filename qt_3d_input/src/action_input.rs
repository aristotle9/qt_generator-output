/// C++ type: <span style='color: green;'>```Qt3DInput::QActionInput```</span>
#[repr(C)]
pub struct ActionInput(u8);

impl ActionInput {
  /// C++ method: <span style='color: green;'>```QVector<int> Qt3DInput::QActionInput::buttons() const```</span>
  ///
  ///
  pub fn buttons(&self) -> ::qt_core::vector::VectorCInt {
    {
      let mut object: ::qt_core::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_buttons_to_output(self as *const ::action_input::ActionInput,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QActionInput::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_metaObject(self as *const ::action_input::ActionInput) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QActionInput::QActionInput()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::action_input::ActionInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QActionInput::QActionInput(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::action_input::ActionInput> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QActionInput::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_qt_metacall(self as *mut ::action_input::ActionInput,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QActionInput::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_qt_metacast(self as *mut ::action_input::ActionInput, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QActionInput::setButtons(const QVector<int>& buttons)```</span>
  ///
  ///
  pub fn set_buttons(&mut self, buttons: &::qt_core::vector::VectorCInt) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_setButtons(self as *mut ::action_input::ActionInput,
                                                             buttons as *const ::qt_core::vector::VectorCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QActionInput::setSourceDevice(Qt3DInput::QAbstractPhysicalDevice* sourceDevice)```</span>
  ///
  ///
  pub unsafe fn set_source_device(&mut self, source_device: *mut ::abstract_physical_device::AbstractPhysicalDevice) {
    ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_setSourceDevice(self as *mut ::action_input::ActionInput, source_device)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractPhysicalDevice* Qt3DInput::QActionInput::sourceDevice() const```</span>
  ///
  ///
  pub fn source_device(&self) -> *mut ::abstract_physical_device::AbstractPhysicalDevice {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_sourceDevice(self as *const ::action_input::ActionInput) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QActionInput::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QActionInput::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::action_input::ActionInput {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QActionInput_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ActionInput`.
  pub struct Signals<'a>(&'a ::action_input::ActionInput);
  /// Represents a built-in Qt signal `Qt3DInput::QActionInput::buttonsChanged`.
  ///
  /// An object of this type can be created from `ActionInput` with `object.signals().buttons_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionInput` object.
  pub struct ButtonsChanged<'a>(&'a ::action_input::ActionInput);
  impl<'a> ::qt_core::connection::Receiver for ButtonsChanged<'a> {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2buttonsChanged(const QVector< int >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ButtonsChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QActionInput::sourceDeviceChanged`.
  ///
  /// An object of this type can be created from `ActionInput` with `object.signals().source_device_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionInput` object.
  pub struct SourceDeviceChanged<'a>(&'a ::action_input::ActionInput);
  impl<'a> ::qt_core::connection::Receiver for SourceDeviceChanged<'a> {
    type Arguments = (*mut ::abstract_physical_device::AbstractPhysicalDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceDeviceChanged(Qt3DInput::QAbstractPhysicalDevice*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceDeviceChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QActionInput::buttonsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn buttons_changed(&self) -> ButtonsChanged {
      ButtonsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QActionInput::sourceDeviceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_device_changed(&self) -> SourceDeviceChanged {
      SourceDeviceChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ActionInput`.
  pub struct Slots<'a>(&'a ::action_input::ActionInput);
  /// Represents a built-in Qt slot `Qt3DInput::QActionInput::setButtons`.
  ///
  /// An object of this type can be created from `ActionInput` with `object.slots().set_buttons()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionInput` object.
  pub struct SetButtons<'a>(&'a ::action_input::ActionInput);
  impl<'a> ::qt_core::connection::Receiver for SetButtons<'a> {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setButtons(const QVector< int >&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QActionInput::setSourceDevice`.
  ///
  /// An object of this type can be created from `ActionInput` with `object.slots().set_source_device()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ActionInput` object.
  pub struct SetSourceDevice<'a>(&'a ::action_input::ActionInput);
  impl<'a> ::qt_core::connection::Receiver for SetSourceDevice<'a> {
    type Arguments = (*mut ::abstract_physical_device::AbstractPhysicalDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceDevice(Qt3DInput::QAbstractPhysicalDevice*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QActionInput::setButtons`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_buttons(&self) -> SetButtons {
      SetButtons(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QActionInput::setSourceDevice`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_device(&self) -> SetSourceDevice {
      SetSourceDevice(self.0)
    }
  }
  impl ::action_input::ActionInput {
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

impl ::cpp_utils::DynamicCast<::action_input::ActionInput> for ::abstract_action_input::AbstractActionInput {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::action_input::ActionInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_dynamic_cast_Qt3DInput_QActionInput_ptr(self as *mut ::abstract_action_input::AbstractActionInput) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::action_input::ActionInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_dynamic_cast_Qt3DInput_QActionInput_ptr(self as *const ::abstract_action_input::AbstractActionInput as *mut ::abstract_action_input::AbstractActionInput) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::action_input::ActionInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::action_input::ActionInput)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::action_input::ActionInput as *mut ::action_input::ActionInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_action_input::AbstractActionInput> for ::action_input::ActionInput {
  fn static_cast_mut(&mut self) -> &mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *mut ::action_input::ActionInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *const ::action_input::ActionInput as *mut ::action_input::ActionInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::action_input::ActionInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QActionInput_G_static_cast_QObject_ptr(self as *mut ::action_input::ActionInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_static_cast_QObject_ptr(self as *const ::action_input::ActionInput as *mut ::action_input::ActionInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::action_input::ActionInput> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::action_input::ActionInput {
    let ffi_result = ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::action_input::ActionInput {
    let ffi_result = ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::action_input::ActionInput> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::action_input::ActionInput {
    let ffi_result = ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::action_input::ActionInput {
    let ffi_result = ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::action_input::ActionInput> for ::abstract_action_input::AbstractActionInput {
  unsafe fn static_cast_mut(&mut self) -> &mut ::action_input::ActionInput {
    let ffi_result = ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DInput_QAbstractActionInput(self as *mut ::abstract_action_input::AbstractActionInput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::action_input::ActionInput {
    let ffi_result = ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QActionInput_ptr_Qt3DInput_QAbstractActionInput(self as *const ::abstract_action_input::AbstractActionInput as *mut ::abstract_action_input::AbstractActionInput);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::action_input::ActionInput {
  type Target = ::abstract_action_input::AbstractActionInput;
  fn deref(&self) -> &::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *const ::action_input::ActionInput as *mut ::action_input::ActionInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::action_input::ActionInput {
  fn deref_mut(&mut self) -> &mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QActionInput_G_static_cast_Qt3DInput_QAbstractActionInput_ptr(self as *mut ::action_input::ActionInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
