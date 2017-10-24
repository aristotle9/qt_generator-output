/// C++ type: <span style='color: green;'>```Qt3DInput::QAnalogAxisInput```</span>
#[repr(C)]
pub struct AnalogAxisInput(u8);

impl AnalogAxisInput {
  /// C++ method: <span style='color: green;'>```int Qt3DInput::QAnalogAxisInput::axis() const```</span>
  ///
  ///
  pub fn axis(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_axis(self as *const ::analog_axis_input::AnalogAxisInput) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QAnalogAxisInput::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_metaObject(self as *const ::analog_axis_input::AnalogAxisInput)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAnalogAxisInput::QAnalogAxisInput()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::analog_axis_input::AnalogAxisInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAnalogAxisInput::QAnalogAxisInput(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::analog_axis_input::AnalogAxisInput> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAnalogAxisInput::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_qt_metacall(self as *mut ::analog_axis_input::AnalogAxisInput,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QAnalogAxisInput::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_qt_metacast(self as *mut ::analog_axis_input::AnalogAxisInput, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAnalogAxisInput::setAxis(int axis)```</span>
  ///
  ///
  pub fn set_axis(&mut self, axis: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_setAxis(self as *mut ::analog_axis_input::AnalogAxisInput, axis)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAnalogAxisInput::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAnalogAxisInput::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::analog_axis_input::AnalogAxisInput {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QAnalogAxisInput_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AnalogAxisInput`.
  pub struct Signals<'a>(&'a ::analog_axis_input::AnalogAxisInput);
  /// Represents a built-in Qt signal `Qt3DInput::QAnalogAxisInput::axisChanged`.
  ///
  /// An object of this type can be created from `AnalogAxisInput` with `object.signals().axis_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnalogAxisInput` object.
  pub struct AxisChanged<'a>(&'a ::analog_axis_input::AnalogAxisInput);
  impl<'a> ::qt_core::connection::Receiver for AxisChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2axisChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AxisChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAnalogAxisInput::sourceDeviceChanged`.
  ///
  /// An object of this type can be created from `AnalogAxisInput` with `object.signals().source_device_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnalogAxisInput` object.
  pub struct SourceDeviceChanged<'a>(&'a ::analog_axis_input::AnalogAxisInput);
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
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAnalogAxisInput::axisChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn axis_changed(&self) -> AxisChanged {
      AxisChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAnalogAxisInput::sourceDeviceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_device_changed(&self) -> SourceDeviceChanged {
      SourceDeviceChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AnalogAxisInput`.
  pub struct Slots<'a>(&'a ::analog_axis_input::AnalogAxisInput);
  /// Represents a built-in Qt slot `Qt3DInput::QAnalogAxisInput::setAxis`.
  ///
  /// An object of this type can be created from `AnalogAxisInput` with `object.slots().set_axis()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnalogAxisInput` object.
  pub struct SetAxis<'a>(&'a ::analog_axis_input::AnalogAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetAxis<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAxis(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAnalogAxisInput::setSourceDevice`.
  ///
  /// An object of this type can be created from `AnalogAxisInput` with `object.slots().set_source_device()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnalogAxisInput` object.
  pub struct SetSourceDevice<'a>(&'a ::analog_axis_input::AnalogAxisInput);
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
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAnalogAxisInput::setAxis`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_axis(&self) -> SetAxis {
      SetAxis(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAnalogAxisInput::setSourceDevice`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_device(&self) -> SetSourceDevice {
      SetSourceDevice(self.0)
    }
  }
  impl ::analog_axis_input::AnalogAxisInput {
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

impl ::cpp_utils::DynamicCast<::analog_axis_input::AnalogAxisInput> for ::abstract_axis_input::AbstractAxisInput {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::analog_axis_input::AnalogAxisInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_dynamic_cast_Qt3DInput_QAnalogAxisInput_ptr(self as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::analog_axis_input::AnalogAxisInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_dynamic_cast_Qt3DInput_QAnalogAxisInput_ptr(self as *const ::abstract_axis_input::AbstractAxisInput as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::analog_axis_input::AnalogAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::analog_axis_input::AnalogAxisInput as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_axis_input::AbstractAxisInput> for ::analog_axis_input::AnalogAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *const ::analog_axis_input::AnalogAxisInput as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::analog_axis_input::AnalogAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_QObject_ptr(self as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_QObject_ptr(self as *const ::analog_axis_input::AnalogAxisInput as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::analog_axis_input::AnalogAxisInput> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::analog_axis_input::AnalogAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::analog_axis_input::AnalogAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::analog_axis_input::AnalogAxisInput> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::analog_axis_input::AnalogAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::analog_axis_input::AnalogAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::analog_axis_input::AnalogAxisInput> for ::abstract_axis_input::AbstractAxisInput {
unsafe fn static_cast_mut(&mut self) -> &mut ::analog_axis_input::AnalogAxisInput {
let ffi_result = ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_Qt3DInput_QAbstractAxisInput(self as *mut ::abstract_axis_input::AbstractAxisInput);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::analog_axis_input::AnalogAxisInput {
let ffi_result = ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAnalogAxisInput_ptr_Qt3DInput_QAbstractAxisInput(self as *const ::abstract_axis_input::AbstractAxisInput as *mut ::abstract_axis_input::AbstractAxisInput);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::analog_axis_input::AnalogAxisInput {
  type Target = ::abstract_axis_input::AbstractAxisInput;
  fn deref(&self) -> &::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *const ::analog_axis_input::AnalogAxisInput as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::analog_axis_input::AnalogAxisInput {
  fn deref_mut(&mut self) -> &mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAnalogAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *mut ::analog_axis_input::AnalogAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
