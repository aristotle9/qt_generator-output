/// C++ type: <span style='color: green;'>```Qt3DRender::QMemoryBarrier```</span>
#[repr(C)]
pub struct MemoryBarrier(u8);

impl MemoryBarrier {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QMemoryBarrier::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_metaObject(self as *const ::memory_barrier::MemoryBarrier)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMemoryBarrier::QMemoryBarrier()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::memory_barrier::MemoryBarrier> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMemoryBarrier::QMemoryBarrier(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::memory_barrier::MemoryBarrier> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QMemoryBarrier::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_qt_metacall(self as *mut ::memory_barrier::MemoryBarrier,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QMemoryBarrier::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_qt_metacast(self as *mut ::memory_barrier::MemoryBarrier, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMemoryBarrier::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMemoryBarrier::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<Qt3DRender::QMemoryBarrier::Operation> Qt3DRender::QMemoryBarrier::waitOperations() const```</span>
  ///
  ///
  pub fn wait_operations(&self) -> ::qt_core::flags::Flags<::memory_barrier::Operation> {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_waitOperations(self as *const ::memory_barrier::MemoryBarrier)
      };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }
}

impl ::cpp_utils::CppDeletable for ::memory_barrier::MemoryBarrier {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QMemoryBarrier_delete
  }
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QMemoryBarrier::Operation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Operation {
  /// C++ enum variant: <span style='color: green;'>```All = -1```</span>
  All = -1,
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```VertexAttributeArray = 1```</span>
  VertexAttributeArray = 1,
  /// C++ enum variant: <span style='color: green;'>```ElementArray = 2```</span>
  ElementArray = 2,
  /// C++ enum variant: <span style='color: green;'>```Uniform = 4```</span>
  Uniform = 4,
  /// C++ enum variant: <span style='color: green;'>```TextureFetch = 8```</span>
  TextureFetch = 8,
  /// C++ enum variant: <span style='color: green;'>```ShaderImageAccess = 16```</span>
  ShaderImageAccess = 16,
  /// C++ enum variant: <span style='color: green;'>```Command = 32```</span>
  Command = 32,
  /// C++ enum variant: <span style='color: green;'>```PixelBuffer = 64```</span>
  PixelBuffer = 64,
  /// C++ enum variant: <span style='color: green;'>```TextureUpdate = 128```</span>
  TextureUpdate = 128,
  /// C++ enum variant: <span style='color: green;'>```BufferUpdate = 256```</span>
  BufferUpdate = 256,
  /// C++ enum variant: <span style='color: green;'>```FrameBuffer = 512```</span>
  FrameBuffer = 512,
  /// C++ enum variant: <span style='color: green;'>```TransformFeedback = 1024```</span>
  TransformFeedback = 1024,
  /// C++ enum variant: <span style='color: green;'>```AtomicCounter = 2048```</span>
  AtomicCounter = 2048,
  /// C++ enum variant: <span style='color: green;'>```ShaderStorage = 4096```</span>
  ShaderStorage = 4096,
  /// C++ enum variant: <span style='color: green;'>```QueryBuffer = 8192```</span>
  QueryBuffer = 8192,
}

impl ::qt_core::flags::FlaggableEnum for Operation {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Operation"
  }
}

impl ::cpp_utils::DynamicCast<::memory_barrier::MemoryBarrier> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::memory_barrier::MemoryBarrier> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_dynamic_cast_Qt3DRender_QMemoryBarrier_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::memory_barrier::MemoryBarrier> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_dynamic_cast_Qt3DRender_QMemoryBarrier_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::memory_barrier::MemoryBarrier {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::memory_barrier::MemoryBarrier) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::memory_barrier::MemoryBarrier as *mut ::memory_barrier::MemoryBarrier) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::memory_barrier::MemoryBarrier {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::memory_barrier::MemoryBarrier) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::memory_barrier::MemoryBarrier as *mut ::memory_barrier::MemoryBarrier) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::memory_barrier::MemoryBarrier {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_QObject_ptr(self as *mut ::memory_barrier::MemoryBarrier)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_QObject_ptr(self as *const ::memory_barrier::MemoryBarrier as *mut ::memory_barrier::MemoryBarrier) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::memory_barrier::MemoryBarrier> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::memory_barrier::MemoryBarrier {
    let ffi_result = ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::memory_barrier::MemoryBarrier {
    let ffi_result = ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::memory_barrier::MemoryBarrier> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::memory_barrier::MemoryBarrier {
    let ffi_result = ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::memory_barrier::MemoryBarrier {
    let ffi_result = ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::memory_barrier::MemoryBarrier> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::memory_barrier::MemoryBarrier {
    let ffi_result = ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::memory_barrier::MemoryBarrier {
    let ffi_result = ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QMemoryBarrier_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::memory_barrier::MemoryBarrier {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::memory_barrier::MemoryBarrier as *mut ::memory_barrier::MemoryBarrier) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::memory_barrier::MemoryBarrier {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMemoryBarrier_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::memory_barrier::MemoryBarrier) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
