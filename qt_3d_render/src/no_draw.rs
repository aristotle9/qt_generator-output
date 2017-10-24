/// C++ type: <span style='color: green;'>```Qt3DRender::QNoDraw```</span>
#[repr(C)]
pub struct NoDraw(u8);

impl NoDraw {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QNoDraw::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_metaObject(self as *const ::no_draw::NoDraw) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QNoDraw::QNoDraw()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::no_draw::NoDraw> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QNoDraw::QNoDraw(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::no_draw::NoDraw> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QNoDraw::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_qt_metacall(self as *mut ::no_draw::NoDraw,
                                                         arg1 as *const ::qt_core::meta_object::Call,
                                                         arg2,
                                                         arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QNoDraw::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_qt_metacast(self as *mut ::no_draw::NoDraw, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QNoDraw::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QNoDraw::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::no_draw::NoDraw {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QNoDraw_delete
  }
}

impl ::cpp_utils::DynamicCast<::no_draw::NoDraw> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::no_draw::NoDraw> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_dynamic_cast_Qt3DRender_QNoDraw_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::no_draw::NoDraw> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_dynamic_cast_Qt3DRender_QNoDraw_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::no_draw::NoDraw {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::no_draw::NoDraw) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::no_draw::NoDraw as *mut ::no_draw::NoDraw) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::no_draw::NoDraw {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::no_draw::NoDraw)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::no_draw::NoDraw as *mut ::no_draw::NoDraw) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::no_draw::NoDraw {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_QObject_ptr(self as *mut ::no_draw::NoDraw) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_QObject_ptr(self as *const ::no_draw::NoDraw as *mut ::no_draw::NoDraw) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::no_draw::NoDraw> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::no_draw::NoDraw {
    let ffi_result = ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QNoDraw_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::no_draw::NoDraw {
    let ffi_result = ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QNoDraw_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::no_draw::NoDraw> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::no_draw::NoDraw {
    let ffi_result = ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QNoDraw_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::no_draw::NoDraw {
    let ffi_result = ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QNoDraw_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::no_draw::NoDraw> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::no_draw::NoDraw {
    let ffi_result = ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QNoDraw_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::no_draw::NoDraw {
    let ffi_result = ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QNoDraw_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::no_draw::NoDraw {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::no_draw::NoDraw as *mut ::no_draw::NoDraw) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::no_draw::NoDraw {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QNoDraw_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::no_draw::NoDraw)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
