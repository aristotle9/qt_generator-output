/// C++ type: <span style='color: green;'>```Qt3DInput::QMouseDevice::Axis```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Axis {
  /// C++ enum variant: <span style='color: green;'>```X = 0```</span>
  X = 0,
  /// C++ enum variant: <span style='color: green;'>```Y = 1```</span>
  Y = 1,
  /// C++ enum variant: <span style='color: green;'>```WheelX = 2```</span>
  WheelX = 2,
  /// C++ enum variant: <span style='color: green;'>```WheelY = 3```</span>
  WheelY = 3,
}

/// C++ type: <span style='color: green;'>```Qt3DInput::QMouseDevice```</span>
#[repr(C)]
pub struct MouseDevice(u8);

impl MouseDevice {
  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QMouseDevice::axisCount() const```</span>
  ///
  ///
  pub fn axis_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_axisCount(self as *const ::mouse_device::MouseDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QMouseDevice::axisIdentifier(const QString& name) const```</span>
  ///
  ///
  pub fn axis_identifier(&self, name: &::qt_core::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_axisIdentifier(self as *const ::mouse_device::MouseDevice,
                                                                 name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList Qt3DInput::QMouseDevice::axisNames() const```</span>
  ///
  ///
  pub fn axis_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_axisNames_to_output(self as *const ::mouse_device::MouseDevice,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QMouseDevice::buttonCount() const```</span>
  ///
  ///
  pub fn button_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_buttonCount(self as *const ::mouse_device::MouseDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QMouseDevice::buttonIdentifier(const QString& name) const```</span>
  ///
  ///
  pub fn button_identifier(&self, name: &::qt_core::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_buttonIdentifier(self as *const ::mouse_device::MouseDevice,
                                                                   name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList Qt3DInput::QMouseDevice::buttonNames() const```</span>
  ///
  ///
  pub fn button_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_buttonNames_to_output(self as *const ::mouse_device::MouseDevice,
                                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QMouseDevice::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_metaObject(self as *const ::mouse_device::MouseDevice) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QMouseDevice::QMouseDevice()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::mouse_device::MouseDevice> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QMouseDevice::QMouseDevice(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::mouse_device::MouseDevice> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QMouseDevice::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_qt_metacall(self as *mut ::mouse_device::MouseDevice,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QMouseDevice::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_qt_metacast(self as *mut ::mouse_device::MouseDevice, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DInput::QMouseDevice::sensitivity() const```</span>
  ///
  ///
  pub fn sensitivity(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_sensitivity(self as *const ::mouse_device::MouseDevice) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QMouseDevice::setSensitivity(float value)```</span>
  ///
  ///
  pub fn set_sensitivity(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_setSensitivity(self as *mut ::mouse_device::MouseDevice, value)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QMouseDevice::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QMouseDevice::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::mouse_device::MouseDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseDevice_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MouseDevice`.
  pub struct Signals<'a>(&'a ::mouse_device::MouseDevice);
  /// Represents a built-in Qt signal `Qt3DInput::QMouseDevice::sensitivityChanged`.
  ///
  /// An object of this type can be created from `MouseDevice` with `object.signals().sensitivity_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseDevice` object.
  pub struct SensitivityChanged<'a>(&'a ::mouse_device::MouseDevice);
  impl<'a> ::qt_core::connection::Receiver for SensitivityChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sensitivityChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SensitivityChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseDevice::sensitivityChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sensitivity_changed(&self) -> SensitivityChanged {
      SensitivityChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MouseDevice`.
  pub struct Slots<'a>(&'a ::mouse_device::MouseDevice);
  /// Represents a built-in Qt slot `Qt3DInput::QMouseDevice::setSensitivity`.
  ///
  /// An object of this type can be created from `MouseDevice` with `object.slots().set_sensitivity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseDevice` object.
  pub struct SetSensitivity<'a>(&'a ::mouse_device::MouseDevice);
  impl<'a> ::qt_core::connection::Receiver for SetSensitivity<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSensitivity(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QMouseDevice::setSensitivity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_sensitivity(&self) -> SetSensitivity {
      SetSensitivity(self.0)
    }
  }
  impl ::mouse_device::MouseDevice {
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

impl ::cpp_utils::DynamicCast<::mouse_device::MouseDevice> for ::abstract_physical_device::AbstractPhysicalDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mouse_device::MouseDevice> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_dynamic_cast_Qt3DInput_QMouseDevice_ptr(self as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mouse_device::MouseDevice> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_dynamic_cast_Qt3DInput_QMouseDevice_ptr(self as *const ::abstract_physical_device::AbstractPhysicalDevice as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::mouse_device::MouseDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::mouse_device::MouseDevice)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::mouse_device::MouseDevice as *mut ::mouse_device::MouseDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_physical_device::AbstractPhysicalDevice> for ::mouse_device::MouseDevice {
  fn static_cast_mut(&mut self) -> &mut ::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *mut ::mouse_device::MouseDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *const ::mouse_device::MouseDevice as *mut ::mouse_device::MouseDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::mouse_device::MouseDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_QObject_ptr(self as *mut ::mouse_device::MouseDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_QObject_ptr(self as *const ::mouse_device::MouseDevice as *mut ::mouse_device::MouseDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_device::MouseDevice> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_device::MouseDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_device::MouseDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_device::MouseDevice> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_device::MouseDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_device::MouseDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_device::MouseDevice> for ::abstract_physical_device::AbstractPhysicalDevice {
unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_device::MouseDevice {
let ffi_result = ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DInput_QAbstractPhysicalDevice(self as *mut ::abstract_physical_device::AbstractPhysicalDevice);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::mouse_device::MouseDevice {
let ffi_result = ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QMouseDevice_ptr_Qt3DInput_QAbstractPhysicalDevice(self as *const ::abstract_physical_device::AbstractPhysicalDevice as *mut ::abstract_physical_device::AbstractPhysicalDevice);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::mouse_device::MouseDevice {
  type Target = ::abstract_physical_device::AbstractPhysicalDevice;
  fn deref(&self) -> &::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *const ::mouse_device::MouseDevice as *mut ::mouse_device::MouseDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mouse_device::MouseDevice {
  fn deref_mut(&mut self) -> &mut ::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *mut ::mouse_device::MouseDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
