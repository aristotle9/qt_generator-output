/// C++ type: <span style='color: green;'>```Qt3DCore::QEntity```</span>
#[repr(C)]
pub struct Entity(u8);

impl Entity {
  /// C++ method: <span style='color: green;'>```void Qt3DCore::QEntity::addComponent(Qt3DCore::QComponent* comp)```</span>
  ///
  ///
  pub unsafe fn add_component(&mut self, comp: *mut ::component::Component) {
    ::ffi::qt_3d_core_c_Qt3DCore_QEntity_addComponent(self as *mut ::entity::Entity, comp)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*> Qt3DCore::QEntity::components() const```</span>
  ///
  ///
  pub fn components(&self) -> ::vector::VectorComponentMutPtr {
    {
      let mut object: ::vector::VectorComponentMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QEntity_components_to_output(self as *const ::entity::Entity, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DCore::QEntity::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QEntity_metaObject(self as *const ::entity::Entity) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QEntity::QEntity()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::entity::Entity> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QEntity_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QEntity::QEntity(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::node::Node) -> ::cpp_utils::CppBox<::entity::Entity> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QEntity_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* Qt3DCore::QEntity::parentEntity() const```</span>
  ///
  ///
  pub fn parent_entity(&self) -> *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QEntity_parentEntity(self as *const ::entity::Entity) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DCore::QEntity::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_core_c_Qt3DCore_QEntity_qt_metacall(self as *mut ::entity::Entity,
                                                     arg1 as *const ::qt_core::meta_object::Call,
                                                     arg2,
                                                     arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DCore::QEntity::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_core_c_Qt3DCore_QEntity_qt_metacast(self as *mut ::entity::Entity, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QEntity::removeComponent(Qt3DCore::QComponent* comp)```</span>
  ///
  ///
  pub unsafe fn remove_component(&mut self, comp: *mut ::component::Component) {
    ::ffi::qt_3d_core_c_Qt3DCore_QEntity_removeComponent(self as *mut ::entity::Entity, comp)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QEntity::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QEntity_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QEntity::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QEntity_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::entity::Entity {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QEntity_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Entity`.
  pub struct Signals<'a>(&'a ::entity::Entity);
  /// Represents a built-in Qt signal `Qt3DCore::QEntity::nodeDestroyed`.
  ///
  /// An object of this type can be created from `Entity` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Entity` object.
  pub struct NodeDestroyed<'a>(&'a ::entity::Entity);
  impl<'a> ::qt_core::connection::Receiver for NodeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nodeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NodeDestroyed<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QEntity::enabledChanged`.
  ///
  /// An object of this type can be created from `Entity` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Entity` object.
  pub struct EnabledChanged<'a>(&'a ::entity::Entity);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QEntity::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `Entity` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Entity` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::entity::Entity);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QEntity::parentChanged`.
  ///
  /// An object of this type can be created from `Entity` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Entity` object.
  pub struct ParentChanged<'a>(&'a ::entity::Entity);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QEntity::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QEntity::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QEntity::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QEntity::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Entity`.
  pub struct Slots<'a>(&'a ::entity::Entity);
  /// Represents a built-in Qt slot `Qt3DCore::QEntity::setEnabled`.
  ///
  /// An object of this type can be created from `Entity` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Entity` object.
  pub struct SetEnabled<'a>(&'a ::entity::Entity);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QEntity::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `Entity` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Entity` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::entity::Entity);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QEntity::setParent`.
  ///
  /// An object of this type can be created from `Entity` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Entity` object.
  pub struct SetParent<'a>(&'a ::entity::Entity);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QEntity::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QEntity::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QEntity::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
  }
  impl ::entity::Entity {
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

impl ::cpp_utils::DynamicCast<::entity::Entity> for ::node::Node {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::entity::Entity> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QEntity_G_dynamic_cast_Qt3DCore_QEntity_ptr(self as *mut ::node::Node) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::entity::Entity> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QEntity_G_dynamic_cast_Qt3DCore_QEntity_ptr(self as *const ::node::Node as *mut ::node::Node) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::node::Node> for ::entity::Entity {
  fn static_cast_mut(&mut self) -> &mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::entity::Entity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::entity::Entity as *mut ::entity::Entity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::entity::Entity {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QEntity_G_static_cast_QObject_ptr(self as *mut ::entity::Entity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QEntity_G_static_cast_QObject_ptr(self as *const ::entity::Entity as *mut ::entity::Entity)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::entity::Entity> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::entity::Entity {
    let ffi_result =
      ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::entity::Entity {
    let ffi_result = ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::entity::Entity> for ::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::entity::Entity {
    let ffi_result =
      ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_Qt3DCore_QNode(self as *mut ::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::entity::Entity {
    let ffi_result = ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QEntity_ptr_Qt3DCore_QNode(self as *const ::node::Node as *mut ::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::entity::Entity {
  type Target = ::node::Node;
  fn deref(&self) -> &::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::entity::Entity as *mut ::entity::Entity) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::entity::Entity {
  fn deref_mut(&mut self) -> &mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QEntity_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::entity::Entity) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
