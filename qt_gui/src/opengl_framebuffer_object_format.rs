/// C++ type: <span style='color: green;'>```QOpenGLFramebufferObjectFormat```</span>
#[repr(C)]
pub struct OpenGLFramebufferObjectFormat([u8; ::type_sizes::QT_GUI_OPENGL_FRAMEBUFFER_OBJECT_FORMAT_OPEN_G_L_FRAMEBUFFER_OBJECT_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for OpenGLFramebufferObjectFormat {
  unsafe fn new_uninitialized() -> OpenGLFramebufferObjectFormat {
    OpenGLFramebufferObjectFormat(::std::mem::uninitialized())
  }
}

impl OpenGLFramebufferObjectFormat {
  /// C++ method: <span style='color: green;'>```unsigned int QOpenGLFramebufferObjectFormat::internalTextureFormat() const```</span>
  ///
  ///
  pub fn internal_texture_format(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_internalTextureFormat(self as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLFramebufferObjectFormat::mipmap() const```</span>
  ///
  ///
  pub fn mipmap(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_mipmap(self as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) -> ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObjectFormat::QOpenGLFramebufferObjectFormat(const QOpenGLFramebufferObjectFormat& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat
    where Args: overloading::OpenGLFramebufferObjectFormatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObjectFormat& QOpenGLFramebufferObjectFormat::operator=(const QOpenGLFramebufferObjectFormat& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat)
                             -> &'l0 mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_operator_assign(self as *mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, other as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLFramebufferObjectFormat::operator==(const QOpenGLFramebufferObjectFormat& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_operator_eq(self as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, other as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLFramebufferObjectFormat::operator!=(const QOpenGLFramebufferObjectFormat& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_operator_neq(self as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, other as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLFramebufferObjectFormat::samples() const```</span>
  ///
  ///
  pub fn samples(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_samples(self as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObjectFormat::setAttachment(QOpenGLFramebufferObject::Attachment attachment)```</span>
  ///
  ///
  pub fn set_attachment(&mut self, attachment: &::opengl_framebuffer_object::Attachment) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_setAttachment(self as *mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, attachment as *const ::opengl_framebuffer_object::Attachment) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObjectFormat::setInternalTextureFormat(unsigned int internalTextureFormat)```</span>
  ///
  ///
  pub fn set_internal_texture_format(&mut self, internal_texture_format: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_setInternalTextureFormat(self as *mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, internal_texture_format) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObjectFormat::setMipmap(bool enabled)```</span>
  ///
  ///
  pub fn set_mipmap(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_setMipmap(self as *mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObjectFormat::setSamples(int samples)```</span>
  ///
  ///
  pub fn set_samples(&mut self, samples: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_setSamples(self as *mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, samples) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObjectFormat::setTextureTarget(unsigned int target)```</span>
  ///
  ///
  pub fn set_texture_target(&mut self, target: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_setTextureTarget(self as *mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, target) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QOpenGLFramebufferObjectFormat::textureTarget() const```</span>
  ///
  ///
  pub fn texture_target(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_textureTarget(self as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) }
  }
}

impl Drop for ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QOpenGLFramebufferObjectFormat::~QOpenGLFramebufferObjectFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_destructor(self as *mut ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLFramebufferObjectFormat::new](../struct.OpenGLFramebufferObjectFormat.html#method.new) method.
  pub trait OpenGLFramebufferObjectFormatNewArgs {
    fn exec(self) -> ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat;
  }
  impl OpenGLFramebufferObjectFormatNewArgs for () {
    fn exec(self) -> ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat {

      {
        let mut object: ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLFramebufferObjectFormatNewArgs for &'a ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat {

  fn exec(self, ) -> ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat {
    let other = self;
    {
let mut object: ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObjectFormat_constructor_other(other as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat, &mut object); }object
}
  }
}
}
