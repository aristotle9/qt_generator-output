/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderStateSet```</span>
#[repr(C)]
pub struct RenderStateSet(u8);

impl RenderStateSet {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderStateSet::addRenderState(Qt3DRender::QRenderState* state)```</span>
  ///
  ///
  pub unsafe fn add_render_state(&mut self, state: *mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_addRenderState(self as *mut ::render_state_set::RenderStateSet,
                                                                    state)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderStateSet::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_metaObject(self as *const ::render_state_set::RenderStateSet)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderStateSet::QRenderStateSet()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_state_set::RenderStateSet> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderStateSet::QRenderStateSet(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::render_state_set::RenderStateSet> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderStateSet::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_qt_metacall(self as *mut ::render_state_set::RenderStateSet,
                                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                                 arg2,
                                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderStateSet::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_qt_metacast(self as *mut ::render_state_set::RenderStateSet, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderStateSet::removeRenderState(Qt3DRender::QRenderState* state)```</span>
  ///
  ///
  pub unsafe fn remove_render_state(&mut self, state: *mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_removeRenderState(self as *mut ::render_state_set::RenderStateSet, state)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*> Qt3DRender::QRenderStateSet::renderStates() const```</span>
  ///
  ///
  pub fn render_states(&self) -> ::vector::VectorRenderStateMutPtr {
    {
      let mut object: ::vector::VectorRenderStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_renderStates_to_output(self as *const ::render_state_set::RenderStateSet, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderStateSet::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderStateSet::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_state_set::RenderStateSet {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderStateSet_delete
  }
}

impl ::cpp_utils::DynamicCast<::render_state_set::RenderStateSet> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::render_state_set::RenderStateSet> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_dynamic_cast_Qt3DRender_QRenderStateSet_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::render_state_set::RenderStateSet> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_dynamic_cast_Qt3DRender_QRenderStateSet_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_state_set::RenderStateSet {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_state_set::RenderStateSet) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_state_set::RenderStateSet as *mut ::render_state_set::RenderStateSet) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::render_state_set::RenderStateSet {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_state_set::RenderStateSet) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_state_set::RenderStateSet as *mut ::render_state_set::RenderStateSet) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_state_set::RenderStateSet {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_QObject_ptr(self as *mut ::render_state_set::RenderStateSet)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_QObject_ptr(self as *const ::render_state_set::RenderStateSet as *mut ::render_state_set::RenderStateSet) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_state_set::RenderStateSet> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_state_set::RenderStateSet {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_state_set::RenderStateSet {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_state_set::RenderStateSet> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_state_set::RenderStateSet {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_state_set::RenderStateSet {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_state_set::RenderStateSet> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_state_set::RenderStateSet {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_state_set::RenderStateSet {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QRenderStateSet_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::render_state_set::RenderStateSet {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_state_set::RenderStateSet as *mut ::render_state_set::RenderStateSet) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_state_set::RenderStateSet {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderStateSet_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_state_set::RenderStateSet) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
