/// C++ type: <span style='color: green;'>```Qt3DRender::QTechniqueFilter```</span>
#[repr(C)]
pub struct TechniqueFilter(u8);

impl TechniqueFilter {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTechniqueFilter::addMatch(Qt3DRender::QFilterKey* filterKey)```</span>
  ///
  ///
  pub unsafe fn add_match(&mut self, filter_key: *mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_addMatch(self as *mut ::technique_filter::TechniqueFilter,
                                                               filter_key)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTechniqueFilter::addParameter(Qt3DRender::QParameter* p)```</span>
  ///
  ///
  pub unsafe fn add_parameter(&mut self, p: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_addParameter(self as *mut ::technique_filter::TechniqueFilter, p)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*> Qt3DRender::QTechniqueFilter::matchAll() const```</span>
  ///
  ///
  pub fn match_all(&self) -> ::vector::VectorFilterKeyMutPtr {
    {
      let mut object: ::vector::VectorFilterKeyMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_matchAll_to_output(self as *const ::technique_filter::TechniqueFilter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QTechniqueFilter::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_metaObject(self as *const ::technique_filter::TechniqueFilter)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTechniqueFilter::QTechniqueFilter()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::technique_filter::TechniqueFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTechniqueFilter::QTechniqueFilter(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::technique_filter::TechniqueFilter> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*> Qt3DRender::QTechniqueFilter::parameters() const```</span>
  ///
  ///
  pub fn parameters(&self) -> ::vector::VectorParameterMutPtr {
    {
      let mut object: ::vector::VectorParameterMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_parameters_to_output(self as *const ::technique_filter::TechniqueFilter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QTechniqueFilter::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_qt_metacall(self as *mut ::technique_filter::TechniqueFilter,
                                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                                  arg2,
                                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QTechniqueFilter::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_qt_metacast(self as *mut ::technique_filter::TechniqueFilter,
                                                                  arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTechniqueFilter::removeMatch(Qt3DRender::QFilterKey* filterKey)```</span>
  ///
  ///
  pub unsafe fn remove_match(&mut self, filter_key: *mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_removeMatch(self as *mut ::technique_filter::TechniqueFilter,
                                                                  filter_key)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTechniqueFilter::removeParameter(Qt3DRender::QParameter* p)```</span>
  ///
  ///
  pub unsafe fn remove_parameter(&mut self, p: *mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_removeParameter(self as *mut ::technique_filter::TechniqueFilter, p)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTechniqueFilter::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTechniqueFilter::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::technique_filter::TechniqueFilter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QTechniqueFilter_delete
  }
}

impl ::cpp_utils::DynamicCast<::technique_filter::TechniqueFilter> for ::frame_graph_node::FrameGraphNode {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::technique_filter::TechniqueFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_dynamic_cast_Qt3DRender_QTechniqueFilter_ptr(self as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::technique_filter::TechniqueFilter> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_dynamic_cast_Qt3DRender_QTechniqueFilter_ptr(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::technique_filter::TechniqueFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::technique_filter::TechniqueFilter as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame_graph_node::FrameGraphNode> for ::technique_filter::TechniqueFilter {
  fn static_cast_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::technique_filter::TechniqueFilter as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::technique_filter::TechniqueFilter {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_QObject_ptr(self as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_QObject_ptr(self as *const ::technique_filter::TechniqueFilter as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::technique_filter::TechniqueFilter> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::technique_filter::TechniqueFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::technique_filter::TechniqueFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::technique_filter::TechniqueFilter> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::technique_filter::TechniqueFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::technique_filter::TechniqueFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::technique_filter::TechniqueFilter> for ::frame_graph_node::FrameGraphNode {
  unsafe fn static_cast_mut(&mut self) -> &mut ::technique_filter::TechniqueFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_Qt3DRender_QFrameGraphNode(self as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::technique_filter::TechniqueFilter {
    let ffi_result = ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QTechniqueFilter_ptr_Qt3DRender_QFrameGraphNode(self as *const ::frame_graph_node::FrameGraphNode as *mut ::frame_graph_node::FrameGraphNode);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::technique_filter::TechniqueFilter {
  type Target = ::frame_graph_node::FrameGraphNode;
  fn deref(&self) -> &::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *const ::technique_filter::TechniqueFilter as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::technique_filter::TechniqueFilter {
  fn deref_mut(&mut self) -> &mut ::frame_graph_node::FrameGraphNode {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTechniqueFilter_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::technique_filter::TechniqueFilter) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
