/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureImageDataGenerator```</span>
#[repr(C)]
pub struct TextureImageDataGenerator(u8);

impl TextureImageDataGenerator {
  /// C++ method: <span style='color: green;'>```pure virtual QSharedPointer<Qt3DRender::QTextureImageData> Qt3DRender::QTextureImageDataGenerator::operator()()```</span>
  ///
  ///
  pub fn op_call(&mut self) -> ::shared_pointer::SharedPointerTextureImageData {
    {
      let mut object: ::shared_pointer::SharedPointerTextureImageData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageDataGenerator_operator_call_to_output(self as *mut ::texture_image_data_generator::TextureImageDataGenerator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool Qt3DRender::QTextureImageDataGenerator::operator==(const Qt3DRender::QTextureImageDataGenerator& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::texture_image_data_generator::TextureImageDataGenerator) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageDataGenerator_operator_eq(self as *const ::texture_image_data_generator::TextureImageDataGenerator, other as *const ::texture_image_data_generator::TextureImageDataGenerator) }
  }
}

impl ::cpp_utils::CppDeletable for ::texture_image_data_generator::TextureImageDataGenerator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageDataGenerator_delete
  }
}

impl ::cpp_utils::DynamicCast<::texture_image_data_generator::TextureImageDataGenerator> for ::abstract_functor::AbstractFunctor {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::texture_image_data_generator::TextureImageDataGenerator> {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_dynamic_cast_Qt3DRender_QTextureImageDataGenerator_ptr(self as *mut ::abstract_functor::AbstractFunctor) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::texture_image_data_generator::TextureImageDataGenerator> {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_dynamic_cast_Qt3DRender_QTextureImageDataGenerator_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::abstract_functor::AbstractFunctor> for ::texture_image_data_generator::TextureImageDataGenerator {
fn static_cast_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::texture_image_data_generator::TextureImageDataGenerator) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_functor::AbstractFunctor {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::texture_image_data_generator::TextureImageDataGenerator as *mut ::texture_image_data_generator::TextureImageDataGenerator) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::texture_image_data_generator::TextureImageDataGenerator> for ::abstract_functor::AbstractFunctor {
unsafe fn static_cast_mut(&mut self) -> &mut ::texture_image_data_generator::TextureImageDataGenerator {
let ffi_result = ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QTextureImageDataGenerator_ptr(self as *mut ::abstract_functor::AbstractFunctor);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::texture_image_data_generator::TextureImageDataGenerator {
let ffi_result = ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QTextureImageDataGenerator_ptr(self as *const ::abstract_functor::AbstractFunctor as *mut ::abstract_functor::AbstractFunctor);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::texture_image_data_generator::TextureImageDataGenerator {
  type Target = ::abstract_functor::AbstractFunctor;
  fn deref(&self) -> &::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *const ::texture_image_data_generator::TextureImageDataGenerator as *mut ::texture_image_data_generator::TextureImageDataGenerator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::texture_image_data_generator::TextureImageDataGenerator {
  fn deref_mut(&mut self) -> &mut ::abstract_functor::AbstractFunctor {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImageDataGenerator_G_static_cast_Qt3DRender_QAbstractFunctor_ptr(self as *mut ::texture_image_data_generator::TextureImageDataGenerator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
