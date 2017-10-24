/// C++ type: <span style='color: green;'>```Qt3DRender::QGeometryFactory```</span>
#[repr(C)]
pub struct GeometryFactory(u8);

impl GeometryFactory {
  /// C++ method: <span style='color: green;'>```pure virtual Qt3DRender::QGeometry* Qt3DRender::QGeometryFactory::operator()()```</span>
  ///
  ///
  pub fn op_call(&mut self) -> *mut ::geometry::Geometry {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometryFactory_operator_call(self as *mut ::geometry_factory::GeometryFactory)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool Qt3DRender::QGeometryFactory::operator==(const Qt3DRender::QGeometryFactory& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::geometry_factory::GeometryFactory) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometryFactory_operator_eq(self as *const ::geometry_factory::GeometryFactory, other as *const ::geometry_factory::GeometryFactory)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::geometry_factory::GeometryFactory {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometryFactory_delete
  }
}

impl ::cpp_utils::DynamicCast<::geometry_factory::GeometryFactory> for ::abstract_functor::AbstractFunctor {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::geometry_factory::GeometryFactory> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryFactory_G_dynamic_cast_Qt3DRender_QGeometryFactory_ptr(self as *mut ::abstract_functor::AbstractFunctor) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::geometry_factory::GeometryFactory> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryFactory_G_dynamic_cast_Qt3DRender_QGeometryFactory_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_functor::AbstractFunctor> for ::geometry_factory::GeometryFactory {
  fn static_cast_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryFactory_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::geometry_factory::GeometryFactory) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryFactory_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::geometry_factory::GeometryFactory as *mut ::geometry_factory::GeometryFactory) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::geometry_factory::GeometryFactory> for ::abstract_functor::AbstractFunctor {
  unsafe fn static_cast_mut(&mut self) -> &mut ::geometry_factory::GeometryFactory {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryFactory_G_static_cast_Qt3DRender_QGeometryFactory_ptr(self as *mut ::abstract_functor::AbstractFunctor);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::geometry_factory::GeometryFactory {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryFactory_G_static_cast_Qt3DRender_QGeometryFactory_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::geometry_factory::GeometryFactory {
  type Target = ::abstract_functor::AbstractFunctor;
  fn deref(&self) -> &::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryFactory_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::geometry_factory::GeometryFactory as *mut ::geometry_factory::GeometryFactory) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::geometry_factory::GeometryFactory {
  fn deref_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryFactory_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::geometry_factory::GeometryFactory) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
