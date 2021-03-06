/// C++ type: <span style='color: green;'>```Qt3DCore::QComponent```</span>
#[repr(C)]
pub struct Component(u8);

impl Component {
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*> Qt3DCore::QComponent::entities() const```</span>
  ///
  ///
  pub fn entities(&self) -> ::vector::VectorEntityMutPtr {
    {
      let mut object: ::vector::VectorEntityMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QComponent_entities_to_output(self as *const ::component::Component, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QComponent::isShareable() const```</span>
  ///
  ///
  pub fn is_shareable(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QComponent_isShareable(self as *const ::component::Component) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DCore::QComponent::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QComponent_metaObject(self as *const ::component::Component) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QComponent::QComponent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::component::Component> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QComponent_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QComponent::QComponent(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::node::Node) -> ::cpp_utils::CppBox<::component::Component> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QComponent_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DCore::QComponent::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_core_c_Qt3DCore_QComponent_qt_metacall(self as *mut ::component::Component,
                                                        arg1 as *const ::qt_core::meta_object::Call,
                                                        arg2,
                                                        arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DCore::QComponent::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_core_c_Qt3DCore_QComponent_qt_metacast(self as *mut ::component::Component, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QComponent::setShareable(bool isShareable)```</span>
  ///
  ///
  pub fn set_shareable(&mut self, is_shareable: bool) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QComponent_setShareable(self as *mut ::component::Component, is_shareable) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QComponent::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QComponent_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QComponent::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QComponent_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::component::Component {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QComponent_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Component`.
  pub struct Signals<'a>(&'a ::component::Component);
  /// Represents a built-in Qt signal `Qt3DCore::QComponent::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `Component` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::component::Component);
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
  /// Represents a built-in Qt signal `Qt3DCore::QComponent::nodeDestroyed`.
  ///
  /// An object of this type can be created from `Component` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct NodeDestroyed<'a>(&'a ::component::Component);
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
  /// Represents a built-in Qt signal `Qt3DCore::QComponent::removedFromEntity`.
  ///
  /// An object of this type can be created from `Component` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct RemovedFromEntity<'a>(&'a ::component::Component);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QComponent::parentChanged`.
  ///
  /// An object of this type can be created from `Component` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct ParentChanged<'a>(&'a ::component::Component);
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
  /// Represents a built-in Qt signal `Qt3DCore::QComponent::shareableChanged`.
  ///
  /// An object of this type can be created from `Component` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct ShareableChanged<'a>(&'a ::component::Component);
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
  /// Represents a built-in Qt signal `Qt3DCore::QComponent::enabledChanged`.
  ///
  /// An object of this type can be created from `Component` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct EnabledChanged<'a>(&'a ::component::Component);
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
  /// Represents a built-in Qt signal `Qt3DCore::QComponent::addedToEntity`.
  ///
  /// An object of this type can be created from `Component` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct AddedToEntity<'a>(&'a ::component::Component);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QComponent::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QComponent::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QComponent::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QComponent::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QComponent::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QComponent::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QComponent::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Component`.
  pub struct Slots<'a>(&'a ::component::Component);
  /// Represents a built-in Qt slot `Qt3DCore::QComponent::setShareable`.
  ///
  /// An object of this type can be created from `Component` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct SetShareable<'a>(&'a ::component::Component);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QComponent::setEnabled`.
  ///
  /// An object of this type can be created from `Component` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct SetEnabled<'a>(&'a ::component::Component);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QComponent::setParent`.
  ///
  /// An object of this type can be created from `Component` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct SetParent<'a>(&'a ::component::Component);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QComponent::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `Component` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Component` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::component::Component);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QComponent::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QComponent::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QComponent::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QComponent::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::component::Component {
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

impl ::cpp_utils::DynamicCast<::component::Component> for ::node::Node {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::component::Component> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QComponent_G_dynamic_cast_Qt3DCore_QComponent_ptr(self as *mut ::node::Node) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::component::Component> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponent_G_dynamic_cast_Qt3DCore_QComponent_ptr(self as *const ::node::Node as *mut ::node::Node) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::node::Node> for ::component::Component {
  fn static_cast_mut(&mut self) -> &mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::component::Component) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::component::Component as *mut ::component::Component) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::component::Component {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QComponent_G_static_cast_QObject_ptr(self as *mut ::component::Component) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponent_G_static_cast_QObject_ptr(self as *const ::component::Component as *mut ::component::Component) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::component::Component> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::component::Component {
    let ffi_result = ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::component::Component {
    let ffi_result = ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::component::Component> for ::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::component::Component {
    let ffi_result =
      ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_Qt3DCore_QNode(self as *mut ::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::component::Component {
    let ffi_result = ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QComponent_ptr_Qt3DCore_QNode(self as *const ::node::Node as *mut ::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::component::Component {
  type Target = ::node::Node;
  fn deref(&self) -> &::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::component::Component as *mut ::component::Component) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::component::Component {
  fn deref_mut(&mut self) -> &mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QComponent_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::component::Component) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
