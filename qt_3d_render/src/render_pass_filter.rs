/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderPassFilter```</span>
#[repr(C)]
pub struct RenderPassFilter(u8);

impl RenderPassFilter {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPassFilter::addMatch(Qt3DRender::QFilterKey* filterKey)```</span>
  ///
  ///
  pub unsafe fn add_match(&mut self, filter_key: *mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_addMatch(self as *mut ::render_pass_filter::RenderPassFilter,
                                                                filter_key)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPassFilter::addParameter(Qt3DRender::QParameter* parameter)```</span>
  ///
  ///
  pub unsafe fn add_parameter(&mut self, parameter: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_addParameter(self as *mut ::render_pass_filter::RenderPassFilter, parameter)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*> Qt3DRender::QRenderPassFilter::matchAny() const```</span>
  ///
  ///
  pub fn match_any(&self) -> ::vector::VectorFilterKeyMutPtr {
    {
      let mut object: ::vector::VectorFilterKeyMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_matchAny_to_output(self as *const ::render_pass_filter::RenderPassFilter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderPassFilter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_metaObject(self as *const ::render_pass_filter::RenderPassFilter) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderPassFilter::QRenderPassFilter()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_pass_filter::RenderPassFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderPassFilter::QRenderPassFilter(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::render_pass_filter::RenderPassFilter> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*> Qt3DRender::QRenderPassFilter::parameters() const```</span>
  ///
  ///
  pub fn parameters(&self) -> ::vector::VectorParameterMutPtr {
    {
      let mut object: ::vector::VectorParameterMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_parameters_to_output(self as *const ::render_pass_filter::RenderPassFilter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderPassFilter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_qt_metacall(self as *mut ::render_pass_filter::RenderPassFilter, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderPassFilter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_qt_metacast(self as *mut ::render_pass_filter::RenderPassFilter, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPassFilter::removeMatch(Qt3DRender::QFilterKey* filterKey)```</span>
  ///
  ///
  pub unsafe fn remove_match(&mut self, filter_key: *mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_removeMatch(self as *mut ::render_pass_filter::RenderPassFilter, filter_key)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QRenderPassFilter::removeParameter(Qt3DRender::QParameter* parameter)```</span>
  ///
  ///
  pub unsafe fn remove_parameter(&mut self, parameter: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_removeParameter(self as *mut ::render_pass_filter::RenderPassFilter, parameter)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderPassFilter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderPassFilter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_pass_filter::RenderPassFilter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderPassFilter_delete
  }
}

impl ::cpp_utils::DynamicCast<::render_pass_filter::RenderPassFilter> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::render_pass_filter::RenderPassFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_dynamic_cast_Qt3DRender_QRenderPassFilter_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::render_pass_filter::RenderPassFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_dynamic_cast_Qt3DRender_QRenderPassFilter_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_pass_filter::RenderPassFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_pass_filter::RenderPassFilter as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::render_pass_filter::RenderPassFilter {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_pass_filter::RenderPassFilter as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_pass_filter::RenderPassFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_QObject_ptr(self as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_QObject_ptr(self as *const ::render_pass_filter::RenderPassFilter as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_pass_filter::RenderPassFilter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_pass_filter::RenderPassFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_pass_filter::RenderPassFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_pass_filter::RenderPassFilter> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_pass_filter::RenderPassFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_pass_filter::RenderPassFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_pass_filter::RenderPassFilter> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_pass_filter::RenderPassFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_pass_filter::RenderPassFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QRenderPassFilter_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::render_pass_filter::RenderPassFilter {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::render_pass_filter::RenderPassFilter as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_pass_filter::RenderPassFilter {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderPassFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::render_pass_filter::RenderPassFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
