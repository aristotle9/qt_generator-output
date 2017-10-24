/// C++ type: <span style='color: green;'>```Qt3DCore::QNode```</span>
#[repr(C)]
pub struct Node(u8);

impl Node {
  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNode::blockNotifications(bool block)```</span>
  ///
  ///
  pub fn block_notifications(&mut self, block: bool) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_blockNotifications(self as *mut ::node::Node, block) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*> Qt3DCore::QNode::childNodes() const```</span>
  ///
  ///
  pub fn child_nodes(&self) -> ::vector::VectorNodeMutPtr {
    {
      let mut object: ::vector::VectorNodeMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QNode_childNodes_to_output(self as *const ::node::Node, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QNode::clearPropertyTracking(const QString& propertyName)```</span>
  ///
  ///
  pub fn clear_property_tracking(&mut self, property_name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNode_clearPropertyTracking(self as *mut ::node::Node,
                                                               property_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QNode::clearPropertyTrackings()```</span>
  ///
  ///
  pub fn clear_property_trackings(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_clearPropertyTrackings(self as *mut ::node::Node) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode::PropertyTrackingMode Qt3DCore::QNode::defaultPropertyTrackingMode() const```</span>
  ///
  ///
  pub fn default_property_tracking_mode(&self) -> ::node::PropertyTrackingMode {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_defaultPropertyTrackingMode(self as *const ::node::Node) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QNode::id() const```</span>
  ///
  ///
  pub fn id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QNode_id_to_output(self as *const ::node::Node, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNode::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_isEnabled(self as *const ::node::Node) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DCore::QNode::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_metaObject(self as *const ::node::Node) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QNode::QNode()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::node::Node> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QNode::QNode(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::node::Node) -> ::cpp_utils::CppBox<::node::Node> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QNode_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNode::notificationsBlocked() const```</span>
  ///
  ///
  pub fn notifications_blocked(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_notificationsBlocked(self as *const ::node::Node) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* Qt3DCore::QNode::parentNode() const```</span>
  ///
  ///
  pub fn parent_node(&self) -> *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_parentNode(self as *const ::node::Node) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode::PropertyTrackingMode Qt3DCore::QNode::propertyTracking(const QString& propertyName) const```</span>
  ///
  ///
  pub fn property_tracking(&self, property_name: &::qt_core::string::String) -> ::node::PropertyTrackingMode {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNode_propertyTracking(self as *const ::node::Node,
                                                          property_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DCore::QNode::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_core_c_Qt3DCore_QNode_qt_metacall(self as *mut ::node::Node,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DCore::QNode::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_core_c_Qt3DCore_QNode_qt_metacast(self as *mut ::node::Node, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QNode::setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode mode)```</span>
  ///
  ///
  pub fn set_default_property_tracking_mode(&mut self, mode: ::node::PropertyTrackingMode) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_setDefaultPropertyTrackingMode(self as *mut ::node::Node, mode) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QNode::setEnabled(bool isEnabled)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, is_enabled: bool) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNode_setEnabled(self as *mut ::node::Node, is_enabled) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DCore::QNode::setParent(Qt3DCore::QNode* parent)```</span>
  ///
  ///
  pub unsafe fn set_parent(&mut self, parent: *mut ::node::Node) {
    ::ffi::qt_3d_core_c_Qt3DCore_QNode_setParent(self as *mut ::node::Node, parent)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QNode::setPropertyTracking(const QString& propertyName, Qt3DCore::QNode::PropertyTrackingMode trackMode)```</span>
  ///
  ///
  pub fn set_property_tracking(&mut self,
                               property_name: &::qt_core::string::String,
                               track_mode: ::node::PropertyTrackingMode) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNode_setPropertyTracking(self as *mut ::node::Node,
                                                             property_name as *const ::qt_core::string::String,
                                                             track_mode)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QNode::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QNode_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DCore::QNode::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QNode_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::node::Node {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QNode_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Node`.
  pub struct Signals<'a>(&'a ::node::Node);
  /// Represents a built-in Qt signal `Qt3DCore::QNode::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `Node` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::node::Node);
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
  /// Represents a built-in Qt signal `Qt3DCore::QNode::objectNameChanged`.
  ///
  /// An object of this type can be created from `Node` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct ObjectNameChanged<'a>(&'a ::node::Node);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DCore::QNode::enabledChanged`.
  ///
  /// An object of this type can be created from `Node` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct EnabledChanged<'a>(&'a ::node::Node);
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
  /// Represents a built-in Qt signal `Qt3DCore::QNode::nodeDestroyed`.
  ///
  /// An object of this type can be created from `Node` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct NodeDestroyed<'a>(&'a ::node::Node);
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
  /// Represents a built-in Qt signal `Qt3DCore::QNode::parentChanged`.
  ///
  /// An object of this type can be created from `Node` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct ParentChanged<'a>(&'a ::node::Node);
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
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QNode::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QNode::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QNode::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QNode::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DCore::QNode::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Node`.
  pub struct Slots<'a>(&'a ::node::Node);
  /// Represents a built-in Qt slot `Qt3DCore::QNode::setParent`.
  ///
  /// An object of this type can be created from `Node` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct SetParent<'a>(&'a ::node::Node);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QNode::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `Node` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::node::Node);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DCore::QNode::setEnabled`.
  ///
  /// An object of this type can be created from `Node` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Node` object.
  pub struct SetEnabled<'a>(&'a ::node::Node);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QNode::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QNode::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DCore::QNode::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::node::Node {
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

/// C++ type: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair```</span>
#[repr(C)]
pub struct NodeIdTypePair([u8; ::type_sizes::QT_3D_CORE_NODE_NODE_ID_TYPE_PAIR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for NodeIdTypePair {
  unsafe fn new_uninitialized() -> NodeIdTypePair {
    NodeIdTypePair(::std::mem::uninitialized())
  }
}

impl NodeIdTypePair {
  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& Qt3DCore::QNodeIdTypePair::id() const```</span>
  ///
  ///
  pub fn id<'l0>(&'l0 self) -> &'l0 ::node_id::NodeId {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_id(self as *const ::node::NodeIdTypePair) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId& Qt3DCore::QNodeIdTypePair::id_mut()```</span>
  ///
  ///
  pub fn id_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_id_mut(self as *mut ::node::NodeIdTypePair) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QNodeIdTypePair::QNodeIdTypePair()```</span>
  ///
  ///
  pub fn new() -> ::node::NodeIdTypePair {
    {
      let mut object: ::node::NodeIdTypePair =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QNodeIdTypePair::QNodeIdTypePair(Qt3DCore::QNodeId _id, const QMetaObject* _type)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(id: &::node_id::NodeId,
                           type_: *const ::qt_core::meta_object::MetaObject)
                           -> ::node::NodeIdTypePair {
    {
      let mut object: ::node::NodeIdTypePair = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_constructor__id__type(id as *const ::node_id::NodeId,
                                                                         type_,
                                                                         &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QNodeIdTypePair::set_id(Qt3DCore::QNodeId value)```</span>
  ///
  ///
  pub fn set_id(&mut self, value: &::node_id::NodeId) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_set_id(self as *mut ::node::NodeIdTypePair,
                                                          value as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QNodeIdTypePair::set_type(const QMetaObject* value)```</span>
  ///
  ///
  pub unsafe fn set_type(&mut self, value: *const ::qt_core::meta_object::MetaObject) {
    ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_set_type(self as *mut ::node::NodeIdTypePair, value)
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* Qt3DCore::QNodeIdTypePair::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_type(self as *const ::node::NodeIdTypePair) }
  }
}

impl Drop for ::node::NodeIdTypePair {
  /// C++ method: <span style='color: green;'>```[destructor] void Qt3DCore::QNodeIdTypePair::~Qt3DCore::QNodeIdTypePair()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeIdTypePair_destructor(self as *mut ::node::NodeIdTypePair) }
  }
}

/// C++ type: <span style='color: green;'>```Qt3DCore::QNode::PropertyTrackingMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PropertyTrackingMode {
  /// C++ enum variant: <span style='color: green;'>```TrackFinalValues = 0```</span>
  TrackFinal = 0,
  /// C++ enum variant: <span style='color: green;'>```DontTrackValues = 1```</span>
  DontTrack = 1,
  /// C++ enum variant: <span style='color: green;'>```TrackAllValues = 2```</span>
  TrackAll = 2,
}

/// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::qIdForNode(Qt3DCore::QNode* node)```</span>
///
///
pub unsafe fn id_for_node(node: *mut ::node::Node) -> ::node_id::NodeId {
  {
    let mut object: ::node_id::NodeId = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_3d_core_c_QNode_G_Qt3DCore_qIdForNode_to_output(node, &mut object);
    object
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::node::Node {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNode_G_static_cast_QObject_ptr(self as *mut ::node::Node) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QNode_G_static_cast_QObject_ptr(self as *const ::node::Node as *mut ::node::Node) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::node::Node> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::node::Node {
    let ffi_result =
      ::ffi::qt_3d_core_c_QNode_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::node::Node {
    let ffi_result = ::ffi::qt_3d_core_c_QNode_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::node::Node {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QNode_G_static_cast_QObject_ptr(self as *const ::node::Node as *mut ::node::Node) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::node::Node {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNode_G_static_cast_QObject_ptr(self as *mut ::node::Node) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
