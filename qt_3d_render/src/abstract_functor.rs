/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractFunctor```</span>
#[repr(C)]
pub struct AbstractFunctor(u8);

impl AbstractFunctor {
  /// C++ method: <span style='color: green;'>```pure virtual qintptr Qt3DRender::QAbstractFunctor::id() const```</span>
  ///
  ///
  pub fn id(&self) -> isize {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractFunctor_id(self as *const ::abstract_functor::AbstractFunctor) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_functor::AbstractFunctor {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractFunctor_delete
  }
}
