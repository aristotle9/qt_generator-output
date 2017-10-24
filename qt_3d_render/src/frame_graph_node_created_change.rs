/// C++ type: <span style='color: green;'>```Qt3DRender::QFrameGraphNodeCreatedChangeBase```</span>
#[repr(C)]
pub struct FrameGraphNodeCreatedChangeBase(u8);

impl FrameGraphNodeCreatedChangeBase {
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QFrameGraphNodeCreatedChangeBase::QFrameGraphNodeCreatedChangeBase(const Qt3DRender::QFrameGraphNode* node)```</span>
  ///
  ///
  pub unsafe fn new(node: *const ::frame_graph_node::FrameGraphNode)
                    -> ::cpp_utils::CppBox<::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNodeCreatedChangeBase_new(node);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DRender::QFrameGraphNodeCreatedChangeBase::parentFrameGraphNodeId() const```</span>
  ///
  ///
  pub fn parent_frame_graph_node_id(&self) -> ::qt_3d_core::node_id::NodeId {
    {
      let mut object: ::qt_3d_core::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNodeCreatedChangeBase_parentFrameGraphNodeId_to_output(self as *const ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QFrameGraphNodeCreatedChangeBase_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node_created_change::NodeCreatedChangeBase> for ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::node_created_change::NodeCreatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *const ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase as *mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::scene_change::SceneChange> for ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase as *mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase> for ::qt_3d_core::node_created_change::NodeCreatedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DRender_QFrameGraphNodeCreatedChangeBase_ptr_Qt3DCore_QNodeCreatedChangeBase(self as *mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DRender_QFrameGraphNodeCreatedChangeBase_ptr_Qt3DCore_QNodeCreatedChangeBase(self as *const ::qt_3d_core::node_created_change::NodeCreatedChangeBase as *mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase> for ::qt_3d_core::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DRender_QFrameGraphNodeCreatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::qt_3d_core::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
let ffi_result = ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DRender_QFrameGraphNodeCreatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::qt_3d_core::scene_change::SceneChange as *mut ::qt_3d_core::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
  type Target = ::qt_3d_core::node_created_change::NodeCreatedChangeBase;
  fn deref(&self) -> &::qt_3d_core::node_created_change::NodeCreatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *const ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase as *mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node_created_change::NodeCreatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QFrameGraphNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *mut ::frame_graph_node_created_change::FrameGraphNodeCreatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
