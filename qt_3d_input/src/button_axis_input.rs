/// C++ type: <span style='color: green;'>```Qt3DInput::QButtonAxisInput```</span>
#[repr(C)]
pub struct ButtonAxisInput(u8);

impl ButtonAxisInput {
  /// C++ method: <span style='color: green;'>```float Qt3DInput::QButtonAxisInput::acceleration() const```</span>
  ///
  ///
  pub fn acceleration(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_acceleration(self as *const ::button_axis_input::ButtonAxisInput)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<int> Qt3DInput::QButtonAxisInput::buttons() const```</span>
  ///
  ///
  pub fn buttons(&self) -> ::qt_core::vector::VectorCInt {
    {
      let mut object: ::qt_core::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_buttons_to_output(self as *const ::button_axis_input::ButtonAxisInput, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DInput::QButtonAxisInput::deceleration() const```</span>
  ///
  ///
  pub fn deceleration(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_deceleration(self as *const ::button_axis_input::ButtonAxisInput)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QButtonAxisInput::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_metaObject(self as *const ::button_axis_input::ButtonAxisInput)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QButtonAxisInput::QButtonAxisInput()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::button_axis_input::ButtonAxisInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QButtonAxisInput::QButtonAxisInput(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::button_axis_input::ButtonAxisInput> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QButtonAxisInput::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_qt_metacall(self as *mut ::button_axis_input::ButtonAxisInput,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QButtonAxisInput::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_qt_metacast(self as *mut ::button_axis_input::ButtonAxisInput, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DInput::QButtonAxisInput::scale() const```</span>
  ///
  ///
  pub fn scale(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_scale(self as *const ::button_axis_input::ButtonAxisInput)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QButtonAxisInput::setAcceleration(float acceleration)```</span>
  ///
  ///
  pub fn set_acceleration(&mut self, acceleration: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_setAcceleration(self as *mut ::button_axis_input::ButtonAxisInput, acceleration)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QButtonAxisInput::setButtons(const QVector<int>& buttons)```</span>
  ///
  ///
  pub fn set_buttons(&mut self, buttons: &::qt_core::vector::VectorCInt) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_setButtons(self as *mut ::button_axis_input::ButtonAxisInput,
                                                                 buttons as *const ::qt_core::vector::VectorCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QButtonAxisInput::setDeceleration(float deceleration)```</span>
  ///
  ///
  pub fn set_deceleration(&mut self, deceleration: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_setDeceleration(self as *mut ::button_axis_input::ButtonAxisInput, deceleration)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QButtonAxisInput::setScale(float scale)```</span>
  ///
  ///
  pub fn set_scale(&mut self, scale: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_setScale(self as *mut ::button_axis_input::ButtonAxisInput, scale)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QButtonAxisInput::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QButtonAxisInput::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::button_axis_input::ButtonAxisInput {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QButtonAxisInput_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ButtonAxisInput`.
  pub struct Signals<'a>(&'a ::button_axis_input::ButtonAxisInput);
  /// Represents a built-in Qt signal `Qt3DInput::QButtonAxisInput::sourceDeviceChanged`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.signals().source_device_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct SourceDeviceChanged<'a>(&'a ::button_axis_input::ButtonAxisInput);
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
  /// Represents a built-in Qt signal `Qt3DInput::QButtonAxisInput::accelerationChanged`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.signals().acceleration_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct AccelerationChanged<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for AccelerationChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2accelerationChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AccelerationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QButtonAxisInput::scaleChanged`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.signals().scale_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct ScaleChanged<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for ScaleChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2scaleChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ScaleChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QButtonAxisInput::decelerationChanged`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.signals().deceleration_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct DecelerationChanged<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for DecelerationChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2decelerationChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DecelerationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QButtonAxisInput::buttonsChanged`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.signals().buttons_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct ButtonsChanged<'a>(&'a ::button_axis_input::ButtonAxisInput);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QButtonAxisInput::sourceDeviceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_device_changed(&self) -> SourceDeviceChanged {
      SourceDeviceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QButtonAxisInput::accelerationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn acceleration_changed(&self) -> AccelerationChanged {
      AccelerationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QButtonAxisInput::scaleChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scale_changed(&self) -> ScaleChanged {
      ScaleChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QButtonAxisInput::decelerationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn deceleration_changed(&self) -> DecelerationChanged {
      DecelerationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QButtonAxisInput::buttonsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn buttons_changed(&self) -> ButtonsChanged {
      ButtonsChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ButtonAxisInput`.
  pub struct Slots<'a>(&'a ::button_axis_input::ButtonAxisInput);
  /// Represents a built-in Qt slot `Qt3DInput::QButtonAxisInput::setAcceleration`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.slots().set_acceleration()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct SetAcceleration<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetAcceleration<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAcceleration(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QButtonAxisInput::setScale`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.slots().set_scale()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct SetScale<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetScale<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setScale(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QButtonAxisInput::setSourceDevice`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.slots().set_source_device()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct SetSourceDevice<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetSourceDevice<'a> {
    type Arguments = (*mut ::abstract_physical_device::AbstractPhysicalDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceDevice(Qt3DInput::QAbstractPhysicalDevice*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QButtonAxisInput::setDeceleration`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.slots().set_deceleration()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct SetDeceleration<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetDeceleration<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDeceleration(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QButtonAxisInput::setButtons`.
  ///
  /// An object of this type can be created from `ButtonAxisInput` with `object.slots().set_buttons()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ButtonAxisInput` object.
  pub struct SetButtons<'a>(&'a ::button_axis_input::ButtonAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetButtons<'a> {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setButtons(const QVector< int >&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QButtonAxisInput::setAcceleration`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_acceleration(&self) -> SetAcceleration {
      SetAcceleration(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QButtonAxisInput::setScale`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_scale(&self) -> SetScale {
      SetScale(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QButtonAxisInput::setSourceDevice`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_device(&self) -> SetSourceDevice {
      SetSourceDevice(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QButtonAxisInput::setDeceleration`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_deceleration(&self) -> SetDeceleration {
      SetDeceleration(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QButtonAxisInput::setButtons`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_buttons(&self) -> SetButtons {
      SetButtons(self.0)
    }
  }
  impl ::button_axis_input::ButtonAxisInput {
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

impl ::cpp_utils::DynamicCast<::button_axis_input::ButtonAxisInput> for ::abstract_axis_input::AbstractAxisInput {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::button_axis_input::ButtonAxisInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_dynamic_cast_Qt3DInput_QButtonAxisInput_ptr(self as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::button_axis_input::ButtonAxisInput> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_dynamic_cast_Qt3DInput_QButtonAxisInput_ptr(self as *const ::abstract_axis_input::AbstractAxisInput as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::button_axis_input::ButtonAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::button_axis_input::ButtonAxisInput as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_axis_input::AbstractAxisInput> for ::button_axis_input::ButtonAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *const ::button_axis_input::ButtonAxisInput as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::button_axis_input::ButtonAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_QObject_ptr(self as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_QObject_ptr(self as *const ::button_axis_input::ButtonAxisInput as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::button_axis_input::ButtonAxisInput> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::button_axis_input::ButtonAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::button_axis_input::ButtonAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::button_axis_input::ButtonAxisInput> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::button_axis_input::ButtonAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::button_axis_input::ButtonAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::button_axis_input::ButtonAxisInput> for ::abstract_axis_input::AbstractAxisInput {
unsafe fn static_cast_mut(&mut self) -> &mut ::button_axis_input::ButtonAxisInput {
let ffi_result = ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DInput_QAbstractAxisInput(self as *mut ::abstract_axis_input::AbstractAxisInput);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::button_axis_input::ButtonAxisInput {
let ffi_result = ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QButtonAxisInput_ptr_Qt3DInput_QAbstractAxisInput(self as *const ::abstract_axis_input::AbstractAxisInput as *mut ::abstract_axis_input::AbstractAxisInput);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::button_axis_input::ButtonAxisInput {
  type Target = ::abstract_axis_input::AbstractAxisInput;
  fn deref(&self) -> &::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *const ::button_axis_input::ButtonAxisInput as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::button_axis_input::ButtonAxisInput {
  fn deref_mut(&mut self) -> &mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QButtonAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr(self as *mut ::button_axis_input::ButtonAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
