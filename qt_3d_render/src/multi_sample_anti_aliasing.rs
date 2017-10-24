/// C++ type: <span style='color: green;'>```Qt3DRender::QMultiSampleAntiAliasing```</span>
#[repr(C)]
pub struct MultiSampleAntiAliasing(u8);

impl MultiSampleAntiAliasing {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QMultiSampleAntiAliasing::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_metaObject(self as *const ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMultiSampleAntiAliasing::QMultiSampleAntiAliasing()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::multi_sample_anti_aliasing::MultiSampleAntiAliasing> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMultiSampleAntiAliasing::QMultiSampleAntiAliasing(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::multi_sample_anti_aliasing::MultiSampleAntiAliasing> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QMultiSampleAntiAliasing::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_qt_metacall(self as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QMultiSampleAntiAliasing::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_qt_metacast(self as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMultiSampleAntiAliasing::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMultiSampleAntiAliasing::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QMultiSampleAntiAliasing_delete
  }
}

impl ::cpp_utils::DynamicCast<::multi_sample_anti_aliasing::MultiSampleAntiAliasing> for ::render_state::RenderState {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_dynamic_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr(self as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::multi_sample_anti_aliasing::MultiSampleAntiAliasing> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_dynamic_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr(self as *const ::render_state::RenderState as *mut ::render_state::RenderState) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::multi_sample_anti_aliasing::MultiSampleAntiAliasing as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::render_state::RenderState> for ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
  fn static_cast_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::multi_sample_anti_aliasing::MultiSampleAntiAliasing as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_QObject_ptr(self as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_QObject_ptr(self as *const ::multi_sample_anti_aliasing::MultiSampleAntiAliasing as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::multi_sample_anti_aliasing::MultiSampleAntiAliasing> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
let ffi_result = ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
let ffi_result = ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::multi_sample_anti_aliasing::MultiSampleAntiAliasing> for ::qt_3d_core::node::Node {
unsafe fn static_cast_mut(&mut self) -> &mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
let ffi_result = ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
let ffi_result = ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::multi_sample_anti_aliasing::MultiSampleAntiAliasing> for ::render_state::RenderState {
unsafe fn static_cast_mut(&mut self) -> &mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
let ffi_result = ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr_Qt3DRender_QRenderState(self as *mut ::render_state::RenderState);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
let ffi_result = ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QMultiSampleAntiAliasing_ptr_Qt3DRender_QRenderState(self as *const ::render_state::RenderState as *mut ::render_state::RenderState);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
  type Target = ::render_state::RenderState;
  fn deref(&self) -> &::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QRenderState_ptr(self as *const ::multi_sample_anti_aliasing::MultiSampleAntiAliasing as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::multi_sample_anti_aliasing::MultiSampleAntiAliasing {
  fn deref_mut(&mut self) -> &mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMultiSampleAntiAliasing_G_static_cast_Qt3DRender_QRenderState_ptr(self as *mut ::multi_sample_anti_aliasing::MultiSampleAntiAliasing) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
