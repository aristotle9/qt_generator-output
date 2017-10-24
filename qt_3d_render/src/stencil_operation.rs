/// C++ type: <span style='color: green;'>```Qt3DRender::QStencilOperation```</span>
#[repr(C)]
pub struct StencilOperation(u8);

impl StencilOperation {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments* Qt3DRender::QStencilOperation::back() const```</span>
  ///
  ///
  pub fn back(&self) -> *mut ::stencil_operation_arguments::StencilOperationArguments {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_back(self as *const ::stencil_operation::StencilOperation)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QStencilOperationArguments* Qt3DRender::QStencilOperation::front() const```</span>
  ///
  ///
  pub fn front(&self) -> *mut ::stencil_operation_arguments::StencilOperationArguments {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_front(self as *const ::stencil_operation::StencilOperation)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QStencilOperation::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_metaObject(self as *const ::stencil_operation::StencilOperation) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QStencilOperation::QStencilOperation()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::stencil_operation::StencilOperation> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QStencilOperation::QStencilOperation(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::stencil_operation::StencilOperation> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QStencilOperation::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_qt_metacall(self as *mut ::stencil_operation::StencilOperation,
                                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                                   arg2,
                                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QStencilOperation::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_qt_metacast(self as *mut ::stencil_operation::StencilOperation,
                                                                   arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilOperation::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QStencilOperation::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::stencil_operation::StencilOperation {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QStencilOperation_delete
  }
}

impl ::cpp_utils::DynamicCast<::stencil_operation::StencilOperation> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::stencil_operation::StencilOperation> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_dynamic_cast_Qt3DRender_QStencilOperation_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::stencil_operation::StencilOperation> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_dynamic_cast_Qt3DRender_QStencilOperation_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::stencil_operation::StencilOperation {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::stencil_operation::StencilOperation as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::stencil_operation::StencilOperation {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::stencil_operation::StencilOperation as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::stencil_operation::StencilOperation {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_QObject_ptr(self as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_QObject_ptr(self as *const ::stencil_operation::StencilOperation as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_operation::StencilOperation> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_operation::StencilOperation {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QStencilOperation_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stencil_operation::StencilOperation {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QStencilOperation_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_operation::StencilOperation> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_operation::StencilOperation {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QStencilOperation_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stencil_operation::StencilOperation {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QStencilOperation_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::stencil_operation::StencilOperation> for ::render_state::RenderState {
  unsafe fn static_cast_mut(&mut self) -> &mut ::stencil_operation::StencilOperation {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QStencilOperation_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::stencil_operation::StencilOperation {
    let ffi_result = ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QStencilOperation_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::stencil_operation::StencilOperation {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::stencil_operation::StencilOperation as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::stencil_operation::StencilOperation {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QStencilOperation_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::stencil_operation::StencilOperation) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
