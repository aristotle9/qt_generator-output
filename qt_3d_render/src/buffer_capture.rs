/// C++ type: <span style='color: green;'>```Qt3DRender::QBufferCapture```</span>
#[repr(C)]
pub struct BufferCapture(u8);

impl BufferCapture {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QBufferCapture::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_metaObject(self as *const ::buffer_capture::BufferCapture)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBufferCapture::QBufferCapture()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::buffer_capture::BufferCapture> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QBufferCapture::QBufferCapture(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::buffer_capture::BufferCapture> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QBufferCapture::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_qt_metacall(self as *mut ::buffer_capture::BufferCapture,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QBufferCapture::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_qt_metacast(self as *mut ::buffer_capture::BufferCapture, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBufferCapture::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QBufferCapture::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::buffer_capture::BufferCapture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QBufferCapture_delete
  }
}

impl ::cpp_utils::DynamicCast<::buffer_capture::BufferCapture> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::buffer_capture::BufferCapture> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_dynamic_cast_Qt3DRender_QBufferCapture_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::buffer_capture::BufferCapture> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_dynamic_cast_Qt3DRender_QBufferCapture_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::buffer_capture::BufferCapture {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::buffer_capture::BufferCapture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::buffer_capture::BufferCapture as *mut ::buffer_capture::BufferCapture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::buffer_capture::BufferCapture {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::buffer_capture::BufferCapture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::buffer_capture::BufferCapture as *mut ::buffer_capture::BufferCapture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::buffer_capture::BufferCapture {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_QObject_ptr(self as *mut ::buffer_capture::BufferCapture)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_QObject_ptr(self as *const ::buffer_capture::BufferCapture as *mut ::buffer_capture::BufferCapture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer_capture::BufferCapture> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::buffer_capture::BufferCapture {
    let ffi_result = ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::buffer_capture::BufferCapture {
    let ffi_result = ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer_capture::BufferCapture> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::buffer_capture::BufferCapture {
    let ffi_result = ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::buffer_capture::BufferCapture {
    let ffi_result = ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer_capture::BufferCapture> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::buffer_capture::BufferCapture {
    let ffi_result = ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::buffer_capture::BufferCapture {
    let ffi_result = ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QBufferCapture_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::buffer_capture::BufferCapture {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::buffer_capture::BufferCapture as *mut ::buffer_capture::BufferCapture) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::buffer_capture::BufferCapture {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferCapture_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::buffer_capture::BufferCapture) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
