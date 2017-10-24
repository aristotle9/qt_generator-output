/// C++ type: <span style='color: green;'>```Qt3DRender::QLayerFilter```</span>
#[repr(C)]
pub struct LayerFilter(u8);

impl LayerFilter {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QLayerFilter::addLayer(Qt3DRender::QLayer* layer)```</span>
  ///
  ///
  pub unsafe fn add_layer(&mut self, layer: *mut ::layer::Layer) {
    ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_addLayer(self as *mut ::layer_filter::LayerFilter, layer)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*> Qt3DRender::QLayerFilter::layers() const```</span>
  ///
  ///
  pub fn layers(&self) -> ::vector::VectorLayerMutPtr {
    {
      let mut object: ::vector::VectorLayerMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_layers_to_output(self as *const ::layer_filter::LayerFilter,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QLayerFilter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_metaObject(self as *const ::layer_filter::LayerFilter) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLayerFilter::QLayerFilter()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::layer_filter::LayerFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QLayerFilter::QLayerFilter(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::layer_filter::LayerFilter> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QLayerFilter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_qt_metacall(self as *mut ::layer_filter::LayerFilter,
                                                              arg1 as *const ::qt_core::meta_object::Call,
                                                              arg2,
                                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QLayerFilter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_qt_metacast(self as *mut ::layer_filter::LayerFilter, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QLayerFilter::removeLayer(Qt3DRender::QLayer* layer)```</span>
  ///
  ///
  pub unsafe fn remove_layer(&mut self, layer: *mut ::layer::Layer) {
    ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_removeLayer(self as *mut ::layer_filter::LayerFilter, layer)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QLayerFilter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QLayerFilter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::layer_filter::LayerFilter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QLayerFilter_delete
  }
}

impl ::cpp_utils::DynamicCast<::layer_filter::LayerFilter> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::layer_filter::LayerFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_dynamic_cast_Qt3DRender_QLayerFilter_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::layer_filter::LayerFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_dynamic_cast_Qt3DRender_QLayerFilter_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::layer_filter::LayerFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::layer_filter::LayerFilter)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::layer_filter::LayerFilter as *mut ::layer_filter::LayerFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::layer_filter::LayerFilter {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::layer_filter::LayerFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::layer_filter::LayerFilter as *mut ::layer_filter::LayerFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::layer_filter::LayerFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_QObject_ptr(self as *mut ::layer_filter::LayerFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_QObject_ptr(self as *const ::layer_filter::LayerFilter as *mut ::layer_filter::LayerFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::layer_filter::LayerFilter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::layer_filter::LayerFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::layer_filter::LayerFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::layer_filter::LayerFilter> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::layer_filter::LayerFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::layer_filter::LayerFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::layer_filter::LayerFilter> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::layer_filter::LayerFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::layer_filter::LayerFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QLayerFilter_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::layer_filter::LayerFilter {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::layer_filter::LayerFilter as *mut ::layer_filter::LayerFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::layer_filter::LayerFilter {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QLayerFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::layer_filter::LayerFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
