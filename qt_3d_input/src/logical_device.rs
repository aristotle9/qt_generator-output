/// C++ type: <span style='color: green;'>```Qt3DInput::QLogicalDevice```</span>
#[repr(C)]
pub struct LogicalDevice(u8);

impl LogicalDevice {
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*> Qt3DInput::QLogicalDevice::actions() const```</span>
  ///
  ///
  pub fn actions(&self) -> ::vector::VectorActionMutPtr {
    {
      let mut object: ::vector::VectorActionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_actions_to_output(self as *const ::logical_device::LogicalDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QLogicalDevice::addAction(Qt3DInput::QAction* action)```</span>
  ///
  ///
  pub unsafe fn add_action(&mut self, action: *mut ::action::Action) {
    ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_addAction(self as *mut ::logical_device::LogicalDevice, action)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QLogicalDevice::addAxis(Qt3DInput::QAxis* axis)```</span>
  ///
  ///
  pub unsafe fn add_axis(&mut self, axis: *mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_addAxis(self as *mut ::logical_device::LogicalDevice, axis)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*> Qt3DInput::QLogicalDevice::axes() const```</span>
  ///
  ///
  pub fn axes(&self) -> ::vector::VectorAxisMutPtr {
    {
      let mut object: ::vector::VectorAxisMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_axes_to_output(self as *const ::logical_device::LogicalDevice,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QLogicalDevice::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_metaObject(self as *const ::logical_device::LogicalDevice) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QLogicalDevice::QLogicalDevice()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::logical_device::LogicalDevice> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QLogicalDevice::QLogicalDevice(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::logical_device::LogicalDevice> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QLogicalDevice::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_qt_metacall(self as *mut ::logical_device::LogicalDevice,
                                                              arg1 as *const ::qt_core::meta_object::Call,
                                                              arg2,
                                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QLogicalDevice::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_qt_metacast(self as *mut ::logical_device::LogicalDevice, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QLogicalDevice::removeAction(Qt3DInput::QAction* action)```</span>
  ///
  ///
  pub unsafe fn remove_action(&mut self, action: *mut ::action::Action) {
    ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_removeAction(self as *mut ::logical_device::LogicalDevice, action)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QLogicalDevice::removeAxis(Qt3DInput::QAxis* axis)```</span>
  ///
  ///
  pub unsafe fn remove_axis(&mut self, axis: *mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_removeAxis(self as *mut ::logical_device::LogicalDevice, axis)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QLogicalDevice::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QLogicalDevice::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::logical_device::LogicalDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QLogicalDevice_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `LogicalDevice`.
  pub struct Signals<'a>(&'a ::logical_device::LogicalDevice);
  /// Represents a built-in Qt signal `Qt3DInput::QLogicalDevice::addedToEntity`.
  ///
  /// An object of this type can be created from `LogicalDevice` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LogicalDevice` object.
  pub struct AddedToEntity<'a>(&'a ::logical_device::LogicalDevice);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QLogicalDevice::shareableChanged`.
  ///
  /// An object of this type can be created from `LogicalDevice` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LogicalDevice` object.
  pub struct ShareableChanged<'a>(&'a ::logical_device::LogicalDevice);
  impl<'a> ::qt_core::connection::Receiver for ShareableChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shareableChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShareableChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QLogicalDevice::removedFromEntity`.
  ///
  /// An object of this type can be created from `LogicalDevice` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LogicalDevice` object.
  pub struct RemovedFromEntity<'a>(&'a ::logical_device::LogicalDevice);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QLogicalDevice::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QLogicalDevice::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QLogicalDevice::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `LogicalDevice`.
  pub struct Slots<'a>(&'a ::logical_device::LogicalDevice);
  /// Represents a built-in Qt slot `Qt3DInput::QLogicalDevice::setShareable`.
  ///
  /// An object of this type can be created from `LogicalDevice` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `LogicalDevice` object.
  pub struct SetShareable<'a>(&'a ::logical_device::LogicalDevice);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QLogicalDevice::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::logical_device::LogicalDevice {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::logical_device::LogicalDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::logical_device::LogicalDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::logical_device::LogicalDevice as *mut ::logical_device::LogicalDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::logical_device::LogicalDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::logical_device::LogicalDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::logical_device::LogicalDevice as *mut ::logical_device::LogicalDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::logical_device::LogicalDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_QObject_ptr(self as *mut ::logical_device::LogicalDevice)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_QObject_ptr(self as *const ::logical_device::LogicalDevice as *mut ::logical_device::LogicalDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::logical_device::LogicalDevice> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::logical_device::LogicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::logical_device::LogicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::logical_device::LogicalDevice> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::logical_device::LogicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::logical_device::LogicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::logical_device::LogicalDevice> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::logical_device::LogicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::logical_device::LogicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DInput_QLogicalDevice_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::logical_device::LogicalDevice {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::logical_device::LogicalDevice as *mut ::logical_device::LogicalDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::logical_device::LogicalDevice {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QLogicalDevice_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::logical_device::LogicalDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
