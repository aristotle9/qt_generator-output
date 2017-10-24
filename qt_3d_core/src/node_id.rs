/// C++ type: <span style='color: green;'>```Qt3DCore::QNodeId```</span>
#[repr(C)]
pub struct NodeId([u8; ::type_sizes::QT_3D_CORE_NODE_ID_NODE_ID]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for NodeId {
  unsafe fn new_uninitialized() -> NodeId {
    NodeId(::std::mem::uninitialized())
  }
}

impl NodeId {
  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNodeId::operator bool() const```</span>
  ///
  ///
  pub fn as_bool(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_convert_to_bool(self as *const ::node_id::NodeId) }
  }

  /// C++ method: <span style='color: green;'>```static Qt3DCore::QNodeId Qt3DCore::QNodeId::createId()```</span>
  ///
  ///
  pub fn create_id() -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_createId_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```quint64 Qt3DCore::QNodeId::id() const```</span>
  ///
  ///
  pub fn id(&self) -> u64 {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_id(self as *const ::node_id::NodeId) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNodeId::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_isNull(self as *const ::node_id::NodeId) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QNodeId::QNodeId()```</span>
  ///
  ///
  pub fn new() -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNodeId::operator==(Qt3DCore::QNodeId other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_operator_eq(self as *const ::node_id::NodeId,
                                                       other as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNodeId::operator>(Qt3DCore::QNodeId other) const```</span>
  ///
  ///
  pub fn op_gt(&self, other: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_operator_gt(self as *const ::node_id::NodeId,
                                                       other as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNodeId::operator<(Qt3DCore::QNodeId other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_operator_lt(self as *const ::node_id::NodeId,
                                                       other as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNodeId::operator!=(Qt3DCore::QNodeId other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_operator_neq(self as *const ::node_id::NodeId,
                                                        other as *const ::node_id::NodeId)
    }
  }
}

impl Drop for ::node_id::NodeId {
  /// C++ method: <span style='color: green;'>```[destructor] void Qt3DCore::QNodeId::~Qt3DCore::QNodeId()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeId_destructor(self as *mut ::node_id::NodeId) }
  }
}

/// C++ method: <span style='color: green;'>```Qt3DCore::qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::node_id::NodeId) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int Qt3DCore::qHash(Qt3DCore::QNodeId id)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::node_id::NodeId, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int Qt3DCore::qHash(Qt3DCore::QNodeId id, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDebug Qt3DCore::operator<<(QDebug d, Qt3DCore::QNodeId id)```</span>
///
///
pub fn op_shl(d: &::qt_core::debug::Debug, id: &::node_id::NodeId) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_3d_core_c_QNodeId_G_operator_shl_to_output(d as *const ::qt_core::debug::Debug,
                                                           id as *const ::node_id::NodeId,
                                                           &mut object);
    }
    object
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::node_id::NodeId {
    fn exec(self) -> ::libc::c_uint {
      let id = self;
      unsafe { ::ffi::qt_3d_core_c_QNodeId_G_Qt3DCore_qHash_id(id as *const ::node_id::NodeId) }
    }
  }
  impl<'a> HashArgs for (&'a ::node_id::NodeId, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let id = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_3d_core_c_QNodeId_G_Qt3DCore_qHash_id_seed(id as *const ::node_id::NodeId, seed) }
    }
  }
}
