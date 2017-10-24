/// C++ type: <span style='color: green;'>```Qt3DRender::QBufferDataGenerator```</span>
#[repr(C)]
pub struct BufferDataGenerator(u8);

impl BufferDataGenerator {
  /// C++ method: <span style='color: green;'>```pure virtual QByteArray Qt3DRender::QBufferDataGenerator::operator()()```</span>
  ///
  ///
  pub fn op_call(&mut self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QBufferDataGenerator_operator_call_to_output(self as *mut ::buffer_data_generator::BufferDataGenerator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool Qt3DRender::QBufferDataGenerator::operator==(const Qt3DRender::QBufferDataGenerator& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::buffer_data_generator::BufferDataGenerator) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QBufferDataGenerator_operator_eq(self as *const ::buffer_data_generator::BufferDataGenerator, other as *const ::buffer_data_generator::BufferDataGenerator) }
  }
}

impl ::cpp_utils::CppDeletable for ::buffer_data_generator::BufferDataGenerator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QBufferDataGenerator_delete
  }
}

impl ::cpp_utils::DynamicCast<::buffer_data_generator::BufferDataGenerator> for ::abstract_functor::AbstractFunctor {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::buffer_data_generator::BufferDataGenerator> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferDataGenerator_G_dynamic_cast_Qt3DRender_QBufferDataGenerator_ptr(self as *mut ::abstract_functor::AbstractFunctor) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::buffer_data_generator::BufferDataGenerator> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferDataGenerator_G_dynamic_cast_Qt3DRender_QBufferDataGenerator_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_functor::AbstractFunctor> for ::buffer_data_generator::BufferDataGenerator {
  fn static_cast_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::buffer_data_generator::BufferDataGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::buffer_data_generator::BufferDataGenerator as *mut ::buffer_data_generator::BufferDataGenerator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer_data_generator::BufferDataGenerator> for ::abstract_functor::AbstractFunctor {
unsafe fn static_cast_mut(&mut self) -> &mut ::buffer_data_generator::BufferDataGenerator {
let ffi_result = ::ffi::qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QBufferDataGenerator_ptr(self as *mut ::abstract_functor::AbstractFunctor);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::buffer_data_generator::BufferDataGenerator {
let ffi_result = ::ffi::qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QBufferDataGenerator_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::buffer_data_generator::BufferDataGenerator {
  type Target = ::abstract_functor::AbstractFunctor;
  fn deref(&self) -> &::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::buffer_data_generator::BufferDataGenerator as *mut ::buffer_data_generator::BufferDataGenerator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::buffer_data_generator::BufferDataGenerator {
  fn deref_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QBufferDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::buffer_data_generator::BufferDataGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
