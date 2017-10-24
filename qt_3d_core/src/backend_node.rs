/// C++ type: <span style='color: green;'>```Qt3DCore::QBackendNode```</span>
#[repr(C)]
pub struct BackendNode(u8);

impl BackendNode {
  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QBackendNode::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QBackendNode_isEnabled(self as *const ::backend_node::BackendNode) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QBackendNode::Mode Qt3DCore::QBackendNode::mode() const```</span>
  ///
  ///
  pub fn mode(&self) -> ::backend_node::Mode {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QBackendNode_mode(self as *const ::backend_node::BackendNode) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QBackendNode::QBackendNode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::backend_node::BackendNode>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QBackendNode::QBackendNode()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::backend_node::Mode) -> ::cpp_utils::CppBox<::backend_node::BackendNode>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QBackendNode::QBackendNode(Qt3DCore::QBackendNode::Mode mode = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::backend_node::BackendNode>
    where Args: overloading::BackendNodeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QBackendNode::peerId() const```</span>
  ///
  ///
  pub fn peer_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QBackendNode_peerId_to_output(self as *const ::backend_node::BackendNode,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QBackendNode::setEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QBackendNode_setEnabled(self as *mut ::backend_node::BackendNode, enabled) }
  }
}

impl ::cpp_utils::CppDeletable for ::backend_node::BackendNode {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QBackendNode_delete
  }
}

/// C++ type: <span style='color: green;'>```Qt3DCore::QBackendNodeMapper```</span>
#[repr(C)]
pub struct BackendNodeMapper(u8);

impl BackendNodeMapper {
  /// C++ method: <span style='color: green;'>```pure virtual Qt3DCore::QBackendNode* Qt3DCore::QBackendNodeMapper::create(const QSharedPointer<Qt3DCore::QNodeCreatedChangeBase>& change) const```</span>
  ///
  ///
  pub fn create(&self,
                change: &::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase)
                -> *mut ::backend_node::BackendNode {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QBackendNodeMapper_create(self as *const ::backend_node::BackendNodeMapper, change as *const ::shared_pointer::SharedPointerNodeCreatedChangeNodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void Qt3DCore::QBackendNodeMapper::destroy(Qt3DCore::QNodeId id) const```</span>
  ///
  ///
  pub fn destroy(&self, id: &::node_id::NodeId) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QBackendNodeMapper_destroy(self as *const ::backend_node::BackendNodeMapper,
                                                              id as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual Qt3DCore::QBackendNode* Qt3DCore::QBackendNodeMapper::get(Qt3DCore::QNodeId id) const```</span>
  ///
  ///
  pub fn get(&self, id: &::node_id::NodeId) -> *mut ::backend_node::BackendNode {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QBackendNodeMapper_get(self as *const ::backend_node::BackendNodeMapper,
                                                          id as *const ::node_id::NodeId)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::backend_node::BackendNodeMapper {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QBackendNodeMapper_delete
  }
}

/// C++ type: <span style='color: green;'>```Qt3DCore::QBackendNode::Mode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Mode {
  /// C++ enum variant: <span style='color: green;'>```ReadOnly = 0```</span>
  Only = 0,
  /// C++ enum variant: <span style='color: green;'>```ReadWrite = 1```</span>
  Write = 1,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [BackendNode::new](../struct.BackendNode.html#method.new) method.
  pub trait BackendNodeNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::backend_node::BackendNode>;
  }
  impl BackendNodeNewArgs for ::backend_node::Mode {
    fn exec(self) -> ::cpp_utils::CppBox<::backend_node::BackendNode> {
      let mode = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QBackendNode_new_mode(mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl BackendNodeNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::backend_node::BackendNode> {

      let ffi_result = unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QBackendNode_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
