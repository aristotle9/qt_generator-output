/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderAspect```</span>
#[repr(C)]
pub struct RenderAspect(u8);

impl RenderAspect {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderAspect::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_metaObject(self as *const ::render_aspect::RenderAspect) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderAspect::QRenderAspect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderAspect::QRenderAspect()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::render_aspect::RenderType) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderAspect::QRenderAspect(Qt3DRender::QRenderAspect::RenderType type)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>
    where Args: overloading::RenderAspectNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderAspect::QRenderAspect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderAspect::QRenderAspect(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::render_aspect::RenderType, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderAspect::QRenderAspect(Qt3DRender::QRenderAspect::RenderType type, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>
    where Args: overloading::RenderAspectNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderAspect::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_qt_metacall(self as *mut ::render_aspect::RenderAspect,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderAspect::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_qt_metacast(self as *mut ::render_aspect::RenderAspect, arg1)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderAspect::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderAspect::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_aspect::RenderAspect {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_delete
  }
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderAspect::RenderType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RenderType {
  /// C++ enum variant: <span style='color: green;'>```Synchronous = 0```</span>
  Synchronous = 0,
  /// C++ enum variant: <span style='color: green;'>```Threaded = 1```</span>
  Threaded = 1,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::abstract_aspect::AbstractAspect> for ::render_aspect::RenderAspect {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *mut ::render_aspect::RenderAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *const ::render_aspect::RenderAspect as *mut ::render_aspect::RenderAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_aspect::RenderAspect {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_QObject_ptr(self as *mut ::render_aspect::RenderAspect)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_QObject_ptr(self as *const ::render_aspect::RenderAspect as *mut ::render_aspect::RenderAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_aspect::RenderAspect> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_aspect::RenderAspect {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DRender_QRenderAspect_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_aspect::RenderAspect {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DRender_QRenderAspect_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_aspect::RenderAspect> for ::qt_3d_core::abstract_aspect::AbstractAspect {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_aspect::RenderAspect {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DRender_QRenderAspect_ptr_Qt3DCore_QAbstractAspect(self as *mut ::qt_3d_core::abstract_aspect::AbstractAspect);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_aspect::RenderAspect {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DRender_QRenderAspect_ptr_Qt3DCore_QAbstractAspect(self as *const ::qt_3d_core::abstract_aspect::AbstractAspect as *mut ::qt_3d_core::abstract_aspect::AbstractAspect);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::render_aspect::RenderAspect {
  type Target = ::qt_3d_core::abstract_aspect::AbstractAspect;
  fn deref(&self) -> &::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *const ::render_aspect::RenderAspect as *mut ::render_aspect::RenderAspect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_aspect::RenderAspect {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(self as *mut ::render_aspect::RenderAspect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RenderAspect::new](../struct.RenderAspect.html#method.new) method.
  pub trait RenderAspectNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>;
  }
  impl RenderAspectNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect> {

      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl RenderAspectNewArgs for ::render_aspect::RenderType {
    fn exec(self) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [RenderAspect::new_unsafe](../struct.RenderAspect.html#method.new_unsafe) method.
  pub trait RenderAspectNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect>;
  }
  impl RenderAspectNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect> {
      let parent = self;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl RenderAspectNewUnsafeArgs for (::render_aspect::RenderType, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::render_aspect::RenderAspect> {
      let type_ = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderAspect_new_type_parent(type_, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
