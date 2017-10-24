/// C++ type: <span style='color: green;'>```Qt3DInput::QKeyboardDevice```</span>
#[repr(C)]
pub struct KeyboardDevice(u8);

impl KeyboardDevice {
  /// C++ method: <span style='color: green;'>```Qt3DInput::QKeyboardHandler* Qt3DInput::QKeyboardDevice::activeInput() const```</span>
  ///
  ///
  pub fn active_input(&self) -> *mut ::keyboard_handler::KeyboardHandler {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_activeInput(self as *const ::keyboard_device::KeyboardDevice)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QKeyboardDevice::axisCount() const```</span>
  ///
  ///
  pub fn axis_count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_axisCount(self as *const ::keyboard_device::KeyboardDevice)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QKeyboardDevice::axisIdentifier(const QString& name) const```</span>
  ///
  ///
  pub fn axis_identifier(&self, name: &::qt_core::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_axisIdentifier(self as *const ::keyboard_device::KeyboardDevice,
                                                                    name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList Qt3DInput::QKeyboardDevice::axisNames() const```</span>
  ///
  ///
  pub fn axis_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_axisNames_to_output(self as *const ::keyboard_device::KeyboardDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QKeyboardDevice::buttonCount() const```</span>
  ///
  ///
  pub fn button_count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_buttonCount(self as *const ::keyboard_device::KeyboardDevice)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QKeyboardDevice::buttonIdentifier(const QString& name) const```</span>
  ///
  ///
  pub fn button_identifier(&self, name: &::qt_core::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_buttonIdentifier(self as *const ::keyboard_device::KeyboardDevice, name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList Qt3DInput::QKeyboardDevice::buttonNames() const```</span>
  ///
  ///
  pub fn button_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_buttonNames_to_output(self as *const ::keyboard_device::KeyboardDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QKeyboardDevice::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_metaObject(self as *const ::keyboard_device::KeyboardDevice)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QKeyboardDevice::QKeyboardDevice()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::keyboard_device::KeyboardDevice> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QKeyboardDevice::QKeyboardDevice(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::keyboard_device::KeyboardDevice> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QKeyboardDevice::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_qt_metacall(self as *mut ::keyboard_device::KeyboardDevice,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QKeyboardDevice::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_qt_metacast(self as *mut ::keyboard_device::KeyboardDevice, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QKeyboardDevice::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QKeyboardDevice::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::keyboard_device::KeyboardDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardDevice_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `KeyboardDevice`.
  pub struct Signals<'a>(&'a ::keyboard_device::KeyboardDevice);
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardDevice::activeInputChanged`.
  ///
  /// An object of this type can be created from `KeyboardDevice` with `object.signals().active_input_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardDevice` object.
  pub struct ActiveInputChanged<'a>(&'a ::keyboard_device::KeyboardDevice);
  impl<'a> ::qt_core::connection::Receiver for ActiveInputChanged<'a> {
    type Arguments = (*mut ::keyboard_handler::KeyboardHandler,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activeInputChanged(Qt3DInput::QKeyboardHandler*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ActiveInputChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardDevice::activeInputChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn active_input_changed(&self) -> ActiveInputChanged {
      ActiveInputChanged(self.0)
    }
  }
  impl ::keyboard_device::KeyboardDevice {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::keyboard_device::KeyboardDevice> for ::abstract_physical_device::AbstractPhysicalDevice {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::keyboard_device::KeyboardDevice> {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_dynamic_cast_Qt3DInput_QKeyboardDevice_ptr(self as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::keyboard_device::KeyboardDevice> {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_dynamic_cast_Qt3DInput_QKeyboardDevice_ptr(self as *const ::abstract_physical_device::AbstractPhysicalDevice as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::keyboard_device::KeyboardDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::keyboard_device::KeyboardDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::keyboard_device::KeyboardDevice as *mut ::keyboard_device::KeyboardDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_physical_device::AbstractPhysicalDevice> for ::keyboard_device::KeyboardDevice {
fn static_cast_mut(&mut self) -> &mut ::abstract_physical_device::AbstractPhysicalDevice {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *mut ::keyboard_device::KeyboardDevice) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_physical_device::AbstractPhysicalDevice {
let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *const ::keyboard_device::KeyboardDevice as *mut ::keyboard_device::KeyboardDevice) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::keyboard_device::KeyboardDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_QObject_ptr(self as *mut ::keyboard_device::KeyboardDevice)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_QObject_ptr(self as *const ::keyboard_device::KeyboardDevice as *mut ::keyboard_device::KeyboardDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::keyboard_device::KeyboardDevice> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::keyboard_device::KeyboardDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::keyboard_device::KeyboardDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::keyboard_device::KeyboardDevice> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::keyboard_device::KeyboardDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::keyboard_device::KeyboardDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::keyboard_device::KeyboardDevice> for ::abstract_physical_device::AbstractPhysicalDevice {
unsafe fn static_cast_mut(&mut self) -> &mut ::keyboard_device::KeyboardDevice {
let ffi_result = ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_Qt3DInput_QAbstractPhysicalDevice(self as *mut ::abstract_physical_device::AbstractPhysicalDevice);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::keyboard_device::KeyboardDevice {
let ffi_result = ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QKeyboardDevice_ptr_Qt3DInput_QAbstractPhysicalDevice(self as *const ::abstract_physical_device::AbstractPhysicalDevice as *mut ::abstract_physical_device::AbstractPhysicalDevice);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::keyboard_device::KeyboardDevice {
  type Target = ::abstract_physical_device::AbstractPhysicalDevice;
  fn deref(&self) -> &::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *const ::keyboard_device::KeyboardDevice as *mut ::keyboard_device::KeyboardDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::keyboard_device::KeyboardDevice {
  fn deref_mut(&mut self) -> &mut ::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *mut ::keyboard_device::KeyboardDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
