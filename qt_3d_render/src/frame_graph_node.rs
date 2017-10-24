/// C++ type: <span style='color: green;'>```Qt3DRender::QFrameGraphNode```</span>
#[repr(C)]
pub struct FrameGraphNode(u8);

impl FrameGraphNode {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QFrameGraphNode::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_metaObject(self as *const ::frame_graph_node::FrameGraphNode)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QFrameGraphNode::QFrameGraphNode()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::frame_graph_node::FrameGraphNode> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QFrameGraphNode::QFrameGraphNode(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::frame_graph_node::FrameGraphNode> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFrameGraphNode* Qt3DRender::QFrameGraphNode::parentFrameGraphNode() const```</span>
  ///
  ///
  pub fn parent_frame_graph_node(&self) -> *mut ::frame_graph_node::FrameGraphNode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_parentFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QFrameGraphNode::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_qt_metacall(self as *mut ::frame_graph_node::FrameGraphNode,
                                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                                 arg2,
                                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QFrameGraphNode::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_qt_metacast(self as *mut ::frame_graph_node::FrameGraphNode, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QFrameGraphNode::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QFrameGraphNode::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::frame_graph_node::FrameGraphNode {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNode_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FrameGraphNode`.
  pub struct Signals<'a>(&'a ::frame_graph_node::FrameGraphNode);
  /// Represents a built-in Qt signal `Qt3DRender::QFrameGraphNode::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `FrameGraphNode` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrameGraphNode` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::frame_graph_node::FrameGraphNode);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QFrameGraphNode::enabledChanged`.
  ///
  /// An object of this type can be created from `FrameGraphNode` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrameGraphNode` object.
  pub struct EnabledChanged<'a>(&'a ::frame_graph_node::FrameGraphNode);
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
  /// Represents a built-in Qt signal `Qt3DRender::QFrameGraphNode::parentChanged`.
  ///
  /// An object of this type can be created from `FrameGraphNode` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrameGraphNode` object.
  pub struct ParentChanged<'a>(&'a ::frame_graph_node::FrameGraphNode);
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
  /// Represents a built-in Qt signal `Qt3DRender::QFrameGraphNode::nodeDestroyed`.
  ///
  /// An object of this type can be created from `FrameGraphNode` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrameGraphNode` object.
  pub struct NodeDestroyed<'a>(&'a ::frame_graph_node::FrameGraphNode);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFrameGraphNode::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFrameGraphNode::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFrameGraphNode::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QFrameGraphNode::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `FrameGraphNode`.
  pub struct Slots<'a>(&'a ::frame_graph_node::FrameGraphNode);
  /// Represents a built-in Qt slot `Qt3DRender::QFrameGraphNode::setParent`.
  ///
  /// An object of this type can be created from `FrameGraphNode` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrameGraphNode` object.
  pub struct SetParent<'a>(&'a ::frame_graph_node::FrameGraphNode);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QFrameGraphNode::setEnabled`.
  ///
  /// An object of this type can be created from `FrameGraphNode` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrameGraphNode` object.
  pub struct SetEnabled<'a>(&'a ::frame_graph_node::FrameGraphNode);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QFrameGraphNode::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `FrameGraphNode` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FrameGraphNode` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::frame_graph_node::FrameGraphNode);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFrameGraphNode::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFrameGraphNode::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QFrameGraphNode::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::frame_graph_node::FrameGraphNode {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::frame_graph_node::FrameGraphNode {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::frame_graph_node::FrameGraphNode {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_QObject_ptr(self as *mut ::frame_graph_node::FrameGraphNode)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_QObject_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::frame_graph_node::FrameGraphNode> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DRender_QFrameGraphNode_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DRender_QFrameGraphNode_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::frame_graph_node::FrameGraphNode> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DRender_QFrameGraphNode_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DRender_QFrameGraphNode_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::frame_graph_node::FrameGraphNode {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::frame_graph_node::FrameGraphNode {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNode_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
