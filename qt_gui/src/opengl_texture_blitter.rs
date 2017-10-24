/// C++ type: <span style='color: green;'>```QOpenGLTextureBlitter```</span>
#[repr(C)]
pub struct OpenGLTextureBlitter([u8; ::type_sizes::QT_GUI_OPENGL_TEXTURE_BLITTER_OPEN_G_L_TEXTURE_BLITTER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for OpenGLTextureBlitter {
  unsafe fn new_uninitialized() -> OpenGLTextureBlitter {
    OpenGLTextureBlitter(::std::mem::uninitialized())
  }
}

impl OpenGLTextureBlitter {
  /// C++ method: <span style='color: green;'>```QOpenGLTextureBlitter::bind```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bind(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTextureBlitter::bind()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bind(&mut self, ::libc::c_uint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTextureBlitter::bind(unsigned int target = ?)```</span>
  ///
  ///
  pub fn bind<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureBlitterBindArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QOpenGLTextureBlitter::blit(GLuint texture, const QMatrix4x4& targetTransform, QOpenGLTextureBlitter::Origin sourceOrigin)```</span>
  ///
  ///
  pub fn blit(&mut self,
              texture: u32,
              target_transform: &::matrix_4x4::Matrix4X4,
              source_origin: ::opengl_texture_blitter::Origin) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTextureBlitter_blit(self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter,
                                                 texture,
                                                 target_transform as *const ::matrix_4x4::Matrix4X4,
                                                 source_origin)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTextureBlitter::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTextureBlitter_create(self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTextureBlitter::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTextureBlitter_destroy(self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTextureBlitter::isCreated() const```</span>
  ///
  ///
  pub fn is_created(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTextureBlitter_isCreated(self as *const ::opengl_texture_blitter::OpenGLTextureBlitter)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTextureBlitter::QOpenGLTextureBlitter()```</span>
  ///
  ///
  pub fn new() -> ::opengl_texture_blitter::OpenGLTextureBlitter {
    {
      let mut object: ::opengl_texture_blitter::OpenGLTextureBlitter =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTextureBlitter_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTextureBlitter::release()```</span>
  ///
  ///
  pub fn release(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTextureBlitter_release(self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTextureBlitter::setOpacity(float opacity)```</span>
  ///
  ///
  pub fn set_opacity(&mut self, opacity: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTextureBlitter_setOpacity(self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter,
                                                       opacity)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTextureBlitter::setRedBlueSwizzle(bool swizzle)```</span>
  ///
  ///
  pub fn set_red_blue_swizzle(&mut self, swizzle: bool) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTextureBlitter_setRedBlueSwizzle(self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter, swizzle) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTextureBlitter::supportsExternalOESTarget() const```</span>
  ///
  ///
  pub fn supports_external_o_e_s_target(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTextureBlitter_supportsExternalOESTarget(self as *const ::opengl_texture_blitter::OpenGLTextureBlitter) }
  }

  /// C++ method: <span style='color: green;'>```static QMatrix4x4 QOpenGLTextureBlitter::targetTransform(const QRectF& target, const QRect& viewport)```</span>
  ///
  ///
  pub fn target_transform(target: &::qt_core::rect_f::RectF,
                          viewport: &::qt_core::rect::Rect)
                          -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTextureBlitter_targetTransform_as_ptr(target as *const ::qt_core::rect_f::RectF,
                                                                     viewport as *const ::qt_core::rect::Rect)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl Drop for ::opengl_texture_blitter::OpenGLTextureBlitter {
  /// C++ method: <span style='color: green;'>```[destructor] void QOpenGLTextureBlitter::~QOpenGLTextureBlitter()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTextureBlitter_destructor(self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter)
    }
  }
}

/// C++ type: <span style='color: green;'>```QOpenGLTextureBlitter::Origin```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Origin {
  /// C++ enum variant: <span style='color: green;'>```OriginBottomLeft = 0```</span>
  Bottom = 0,
  /// C++ enum variant: <span style='color: green;'>```OriginTopLeft = 1```</span>
  Top = 1,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLTextureBlitter::bind](../struct.OpenGLTextureBlitter.html#method.bind) method.
  pub trait OpenGLTextureBlitterBindArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture_blitter::OpenGLTextureBlitter) -> ();
  }
  impl<'largs> OpenGLTextureBlitterBindArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::opengl_texture_blitter::OpenGLTextureBlitter) -> () {

      unsafe { ::ffi::qt_gui_c_QOpenGLTextureBlitter_bind_no_args(original_self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter) }
    }
  }
  impl<'largs> OpenGLTextureBlitterBindArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::opengl_texture_blitter::OpenGLTextureBlitter) -> () {
      let target = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLTextureBlitter_bind_target(original_self as *mut ::opengl_texture_blitter::OpenGLTextureBlitter, target) }
    }
  }
}
