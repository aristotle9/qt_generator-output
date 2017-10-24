/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureGenerator```</span>
#[repr(C)]
pub struct TextureGenerator(u8);

impl TextureGenerator {
  /// C++ method: <span style='color: green;'>```pure virtual QSharedPointer<Qt3DRender::QTextureData> Qt3DRender::QTextureGenerator::operator()()```</span>
  ///
  ///
  pub fn op_call(&mut self) -> ::shared_pointer::SharedPointerTextureData {
    {
      let mut object: ::shared_pointer::SharedPointerTextureData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTextureGenerator_operator_call_to_output(self as *mut ::texture_generator::TextureGenerator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool Qt3DRender::QTextureGenerator::operator==(const Qt3DRender::QTextureGenerator& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::texture_generator::TextureGenerator) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureGenerator_operator_eq(self as *const ::texture_generator::TextureGenerator, other as *const ::texture_generator::TextureGenerator) }
  }
}

impl ::cpp_utils::CppDeletable for ::texture_generator::TextureGenerator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureGenerator_delete
  }
}

impl ::cpp_utils::DynamicCast<::texture_generator::TextureGenerator> for ::abstract_functor::AbstractFunctor {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::texture_generator::TextureGenerator> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureGenerator_G_dynamic_cast_Qt3DRender_QTextureGenerator_ptr(self as *mut ::abstract_functor::AbstractFunctor) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::texture_generator::TextureGenerator> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureGenerator_G_dynamic_cast_Qt3DRender_QTextureGenerator_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_functor::AbstractFunctor> for ::texture_generator::TextureGenerator {
  fn static_cast_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::texture_generator::TextureGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::texture_generator::TextureGenerator as *mut ::texture_generator::TextureGenerator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_generator::TextureGenerator> for ::abstract_functor::AbstractFunctor {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_generator::TextureGenerator {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QTextureGenerator_ptr(self as *mut ::abstract_functor::AbstractFunctor);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_generator::TextureGenerator {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QTextureGenerator_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::texture_generator::TextureGenerator {
  type Target = ::abstract_functor::AbstractFunctor;
  fn deref(&self) -> &::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::texture_generator::TextureGenerator as *mut ::texture_generator::TextureGenerator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::texture_generator::TextureGenerator {
  fn deref_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::texture_generator::TextureGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
