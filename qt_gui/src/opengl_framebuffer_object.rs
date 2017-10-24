/// C++ type: <span style='color: green;'>```QOpenGLFramebufferObject::Attachment```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Attachment {
  /// C++ enum variant: <span style='color: green;'>```NoAttachment = 0```</span>
  NoAttachment = 0,
  /// C++ enum variant: <span style='color: green;'>```CombinedDepthStencil = 1```</span>
  CombinedDepthStencil = 1,
  /// C++ enum variant: <span style='color: green;'>```Depth = 2```</span>
  Depth = 2,
}

/// C++ type: <span style='color: green;'>```QOpenGLFramebufferObject::FramebufferRestorePolicy```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FramebufferRestorePolicy {
  /// C++ enum variant: <span style='color: green;'>```DontRestoreFramebufferBinding = 0```</span>
  DontRestoreFramebufferBinding = 0,
  /// C++ enum variant: <span style='color: green;'>```RestoreFramebufferBindingToDefault = 1```</span>
  RestoreFramebufferBindingToDefault = 1,
  /// C++ enum variant: <span style='color: green;'>```RestoreFrameBufferBinding = 2```</span>
  RestoreFrameBufferBinding = 2,
}

/// C++ type: <span style='color: green;'>```QOpenGLFramebufferObject```</span>
#[repr(C)]
pub struct OpenGLFramebufferObject(u8);

impl OpenGLFramebufferObject {
  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObject::addColorAttachment```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_color_attachment(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObject::addColorAttachment(const QSize& size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_color_attachment(&mut self, (&::qt_core::size::Size, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObject::addColorAttachment(const QSize& size, unsigned int internalFormat = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_color_attachment(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObject::addColorAttachment(int width, int height)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_color_attachment(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObject::addColorAttachment(int width, int height, unsigned int internalFormat = ?)```</span>
  ///
  ///
  pub fn add_color_attachment<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLFramebufferObjectAddColorAttachmentArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObject::Attachment QOpenGLFramebufferObject::attachment() const```</span>
  ///
  ///
  pub fn attachment(&self) -> ::opengl_framebuffer_object::Attachment {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_attachment(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLFramebufferObject::bind()```</span>
  ///
  ///
  pub fn bind(&mut self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_bind(self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLFramebufferObject::bindDefault()```</span>
  ///
  ///
  pub fn bind_default() -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_bindDefault() }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObject::blitFramebuffer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, QOpenGLFramebufferObject* source)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, QOpenGLFramebufferObject* source, unsigned int buffers = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, ::libc::c_uint, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, QOpenGLFramebufferObject* source, unsigned int buffers = ?, unsigned int filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, const QRect& targetRect, QOpenGLFramebufferObject* source, const QRect& sourceRect)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, const QRect& targetRect, QOpenGLFramebufferObject* source, const QRect& sourceRect, unsigned int buffers = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, ::libc::c_uint, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, const QRect& targetRect, QOpenGLFramebufferObject* source, const QRect& sourceRect, unsigned int buffers = ?, unsigned int filter = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, ::libc::c_uint, ::libc::c_uint, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, const QRect& targetRect, QOpenGLFramebufferObject* source, const QRect& sourceRect, unsigned int buffers, unsigned int filter, int readColorAttachmentIndex, int drawColorAttachmentIndex)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn blit_framebuffer((*mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, &::qt_core::rect::Rect, ::libc::c_uint, ::libc::c_uint, ::libc::c_int, ::libc::c_int, ::opengl_framebuffer_object::FramebufferRestorePolicy)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QOpenGLFramebufferObject::blitFramebuffer(QOpenGLFramebufferObject* target, const QRect& targetRect, QOpenGLFramebufferObject* source, const QRect& sourceRect, unsigned int buffers, unsigned int filter, int readColorAttachmentIndex, int drawColorAttachmentIndex, QOpenGLFramebufferObject::FramebufferRestorePolicy restorePolicy)```</span>
  ///
  ///
  pub unsafe fn blit_framebuffer<Args>(args: Args) -> ()
    where Args: overloading::OpenGLFramebufferObjectBlitFramebufferArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObjectFormat QOpenGLFramebufferObject::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat {
    {
      let mut object: ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLFramebufferObject_format_to_output(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLFramebufferObject::handle() const```</span>
  ///
  ///
  pub fn handle(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_handle(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLFramebufferObject::hasOpenGLFramebufferBlit()```</span>
  ///
  ///
  pub fn has_opengl_framebuffer_blit() -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_hasOpenGLFramebufferBlit() }
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLFramebufferObject::hasOpenGLFramebufferObjects()```</span>
  ///
  ///
  pub fn has_opengl_framebuffer_objects() -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_hasOpenGLFramebufferObjects() }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLFramebufferObject::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_height(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLFramebufferObject::isBound() const```</span>
  ///
  ///
  pub fn is_bound(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_isBound(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLFramebufferObject::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_isValid(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObject::QOpenGLFramebufferObject```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::qt_core::size::Size) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize& size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, ::opengl_framebuffer_object::Attachment)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize& size, QOpenGLFramebufferObject::Attachment attachment)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, ::opengl_framebuffer_object::Attachment, ::libc::c_uint)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize& size, QOpenGLFramebufferObject::Attachment attachment, unsigned int target = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, ::opengl_framebuffer_object::Attachment, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize& size, QOpenGLFramebufferObject::Attachment attachment, unsigned int target = ?, unsigned int internalFormat = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, &::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize& size, const QOpenGLFramebufferObjectFormat& format)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, ::libc::c_uint)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(const QSize& size, unsigned int target = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::opengl_framebuffer_object::Attachment)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, QOpenGLFramebufferObject::Attachment attachment)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::opengl_framebuffer_object::Attachment, ::libc::c_uint)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, QOpenGLFramebufferObject::Attachment attachment, unsigned int target = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::opengl_framebuffer_object::Attachment, ::libc::c_uint, ::libc::c_uint)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, QOpenGLFramebufferObject::Attachment attachment, unsigned int target = ?, unsigned int internalFormat = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, &::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, const QOpenGLFramebufferObjectFormat& format)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_uint)) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFramebufferObject::QOpenGLFramebufferObject(int width, int height, unsigned int target = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>
    where Args: overloading::OpenGLFramebufferObjectNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QOpenGLFramebufferObject::release()```</span>
  ///
  ///
  pub fn release(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_release(self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFramebufferObject::setAttachment(QOpenGLFramebufferObject::Attachment attachment)```</span>
  ///
  ///
  pub fn set_attachment(&mut self, attachment: ::opengl_framebuffer_object::Attachment) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_setAttachment(self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, attachment) }
  }

  /// C++ method: <span style='color: green;'>```QSize QOpenGLFramebufferObject::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLFramebufferObject_size_to_output(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize> QOpenGLFramebufferObject::sizes() const```</span>
  ///
  ///
  pub fn sizes(&self) -> ::vector::VectorQtCoreSize {
    {
      let mut object: ::vector::VectorQtCoreSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLFramebufferObject_sizes_to_output(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObject::takeTexture```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn take_texture(&mut self, ()) -> u32```<br>
  /// C++ method: <span style='color: green;'>```GLuint QOpenGLFramebufferObject::takeTexture()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn take_texture(&mut self, ::libc::c_int) -> u32```<br>
  /// C++ method: <span style='color: green;'>```GLuint QOpenGLFramebufferObject::takeTexture(int colorAttachmentIndex)```</span>
  ///
  ///
  pub fn take_texture<'largs, Args>(&'largs mut self, args: Args) -> u32
    where Args: overloading::OpenGLFramebufferObjectTakeTextureArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```GLuint QOpenGLFramebufferObject::texture() const```</span>
  ///
  ///
  pub fn texture(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_texture(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint> QOpenGLFramebufferObject::textures() const```</span>
  ///
  ///
  pub fn textures(&self) -> ::vector::VectorU32 {
    {
      let mut object: ::vector::VectorU32 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLFramebufferObject_textures_to_output(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLFramebufferObject::toImage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_image(&self, ()) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QOpenGLFramebufferObject::toImage() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_image(&self, bool) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QOpenGLFramebufferObject::toImage(bool flipped) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_image(&self, (bool, ::libc::c_int)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QOpenGLFramebufferObject::toImage(bool flipped, int colorAttachmentIndex) const```</span>
  ///
  ///
  pub fn to_image<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::OpenGLFramebufferObjectToImageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QOpenGLFramebufferObject::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_width(self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_framebuffer_object::OpenGLFramebufferObject {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLFramebufferObject_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLFramebufferObject::add_color_attachment](../struct.OpenGLFramebufferObject.html#method.add_color_attachment) method.
  pub trait OpenGLFramebufferObjectAddColorAttachmentArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> ();
  }
  impl<'largs> OpenGLFramebufferObjectAddColorAttachmentArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> () {
      let size = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_size(original_self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, size as *const ::qt_core::size::Size) }
    }
  }
  impl<'largs> OpenGLFramebufferObjectAddColorAttachmentArgs<'largs> for (&'largs ::qt_core::size::Size, ::libc::c_uint) {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> () {
      let size = self.0;
      let internal_format = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_size_internalFormat(original_self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, size as *const ::qt_core::size::Size, internal_format) }
    }
  }
  impl<'largs> OpenGLFramebufferObjectAddColorAttachmentArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> () {
      let width = self.0;
      let height = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_width_height(original_self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, width, height) }
    }
  }
  impl<'largs> OpenGLFramebufferObjectAddColorAttachmentArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_uint) {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> () {
      let width = self.0;
      let height = self.1;
      let internal_format = self.2;
      unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_addColorAttachment_width_height_internalFormat(original_self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, width, height, internal_format) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLFramebufferObject::blit_framebuffer](../struct.OpenGLFramebufferObject.html#method.blit_framebuffer) method.
  pub trait OpenGLFramebufferObjectBlitFramebufferArgs {
    unsafe fn exec(self) -> ();
  }
  impl OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                           *mut ::opengl_framebuffer_object::OpenGLFramebufferObject) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let source = self.1;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_source(target, source)
    }
  }
  impl OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                           *mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                           ::libc::c_uint) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let source = self.1;
      let buffers = self.2;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_source_buffers(target, source, buffers)
    }
  }
  impl OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                           *mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                           ::libc::c_uint,
                                                           ::libc::c_uint) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let source = self.1;
      let buffers = self.2;
      let filter = self.3;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_source_buffers_filter(target,
                                                                                            source,
                                                                                            buffers,
                                                                                            filter)
    }
  }
  impl<'a> OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               *mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let target_rect = self.1;
      let source = self.2;
      let source_rect = self.3;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect(target, target_rect as *const ::qt_core::rect::Rect, source, source_rect as *const ::qt_core::rect::Rect)
    }
  }
  impl<'a> OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               *mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               ::libc::c_uint) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let target_rect = self.1;
      let source = self.2;
      let source_rect = self.3;
      let buffers = self.4;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers(target, target_rect as *const ::qt_core::rect::Rect, source, source_rect as *const ::qt_core::rect::Rect, buffers)
    }
  }
  impl<'a> OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               *mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               ::libc::c_uint,
                                                               ::libc::c_uint) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let target_rect = self.1;
      let source = self.2;
      let source_rect = self.3;
      let buffers = self.4;
      let filter = self.5;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers_filter(target, target_rect as *const ::qt_core::rect::Rect, source, source_rect as *const ::qt_core::rect::Rect, buffers, filter)
    }
  }
  impl<'a> OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               *mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               ::libc::c_uint,
                                                               ::libc::c_uint,
                                                               ::libc::c_int,
                                                               ::libc::c_int) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let target_rect = self.1;
      let source = self.2;
      let source_rect = self.3;
      let buffers = self.4;
      let filter = self.5;
      let read_color_attachment_index = self.6;
      let draw_color_attachment_index = self.7;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers_filter_readColorAttachmentIndex_drawColorAttachmentIndex(target, target_rect as *const ::qt_core::rect::Rect, source, source_rect as *const ::qt_core::rect::Rect, buffers, filter, read_color_attachment_index, draw_color_attachment_index)
    }
  }
  impl<'a> OpenGLFramebufferObjectBlitFramebufferArgs
    for (*mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               *mut ::opengl_framebuffer_object::OpenGLFramebufferObject,
                                                               &'a ::qt_core::rect::Rect,
                                                               ::libc::c_uint,
                                                               ::libc::c_uint,
                                                               ::libc::c_int,
                                                               ::libc::c_int,
                                                               ::opengl_framebuffer_object::FramebufferRestorePolicy) {
    unsafe fn exec(self) -> () {
      let target = self.0;
      let target_rect = self.1;
      let source = self.2;
      let source_rect = self.3;
      let buffers = self.4;
      let filter = self.5;
      let read_color_attachment_index = self.6;
      let draw_color_attachment_index = self.7;
      let restore_policy = self.8;
      ::ffi::qt_gui_c_QOpenGLFramebufferObject_blitFramebuffer_target_targetRect_source_sourceRect_buffers_filter_readColorAttachmentIndex_drawColorAttachmentIndex_restorePolicy(target, target_rect as *const ::qt_core::rect::Rect, source, source_rect as *const ::qt_core::rect::Rect, buffers, filter, read_color_attachment_index, draw_color_attachment_index, restore_policy)
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLFramebufferObject::new](../struct.OpenGLFramebufferObject.html#method.new) method.
  pub trait OpenGLFramebufferObjectNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject>;
  }
  impl<'a> OpenGLFramebufferObjectNewArgs for &'a ::qt_core::size::Size {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let size = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_size(size as *const ::qt_core::size::Size) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLFramebufferObjectNewArgs for (&'a ::qt_core::size::Size, ::opengl_framebuffer_object::Attachment) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let size = self.0;
      let attachment = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_size_attachment(size as *const ::qt_core::size::Size, attachment)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLFramebufferObjectNewArgs
    for (&'a ::qt_core::size::Size, ::opengl_framebuffer_object::Attachment, ::libc::c_uint) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let size = self.0;
      let attachment = self.1;
      let target = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_size_attachment_target(size as *const ::qt_core::size::Size,
                                                                              attachment,
                                                                              target)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLFramebufferObjectNewArgs
    for (&'a ::qt_core::size::Size, ::opengl_framebuffer_object::Attachment, ::libc::c_uint, ::libc::c_uint) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let size = self.0;
      let attachment = self.1;
      let target = self.2;
      let internal_format = self.3;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_size_attachment_target_internalFormat(size as *const ::qt_core::size::Size, attachment, target, internal_format) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLFramebufferObjectNewArgs
    for (&'a ::qt_core::size::Size, &'a ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let size = self.0;
      let format = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_size_format(size as *const ::qt_core::size::Size, format as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLFramebufferObjectNewArgs for (&'a ::qt_core::size::Size, ::libc::c_uint) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let size = self.0;
      let target = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_size_target(size as *const ::qt_core::size::Size, target)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLFramebufferObjectNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let width = self.0;
      let height = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_width_height(width, height) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLFramebufferObjectNewArgs for (::libc::c_int, ::libc::c_int, ::opengl_framebuffer_object::Attachment) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let width = self.0;
      let height = self.1;
      let attachment = self.2;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_width_height_attachment(width, height, attachment) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLFramebufferObjectNewArgs
    for (::libc::c_int, ::libc::c_int, ::opengl_framebuffer_object::Attachment, ::libc::c_uint) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let width = self.0;
      let height = self.1;
      let attachment = self.2;
      let target = self.3;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_width_height_attachment_target(width, height, attachment, target)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLFramebufferObjectNewArgs
    for (::libc::c_int, ::libc::c_int, ::opengl_framebuffer_object::Attachment, ::libc::c_uint, ::libc::c_uint) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let width = self.0;
      let height = self.1;
      let attachment = self.2;
      let target = self.3;
      let internal_format = self.4;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_width_height_attachment_target_internalFormat(width,
                                                                                                     height,
                                                                                                     attachment,
                                                                                                     target,
                                                                                                     internal_format)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLFramebufferObjectNewArgs
    for (::libc::c_int, ::libc::c_int, &'a ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let width = self.0;
      let height = self.1;
      let format = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_width_height_format(width, height, format as *const ::opengl_framebuffer_object_format::OpenGLFramebufferObjectFormat) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLFramebufferObjectNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_uint) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_framebuffer_object::OpenGLFramebufferObject> {
      let width = self.0;
      let height = self.1;
      let target = self.2;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_new_width_height_target(width, height, target) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLFramebufferObject::take_texture](../struct.OpenGLFramebufferObject.html#method.take_texture) method.
  pub trait OpenGLFramebufferObjectTakeTextureArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> u32;
  }
  impl<'largs> OpenGLFramebufferObjectTakeTextureArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> u32 {
      let color_attachment_index = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_takeTexture_colorAttachmentIndex(original_self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject, color_attachment_index) }
    }
  }
  impl<'largs> OpenGLFramebufferObjectTakeTextureArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::opengl_framebuffer_object::OpenGLFramebufferObject) -> u32 {

      unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_takeTexture_no_args(original_self as *mut ::opengl_framebuffer_object::OpenGLFramebufferObject) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLFramebufferObject::to_image](../struct.OpenGLFramebufferObject.html#method.to_image) method.
  pub trait OpenGLFramebufferObjectToImageArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::opengl_framebuffer_object::OpenGLFramebufferObject)
            -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> OpenGLFramebufferObjectToImageArgs<'largs> for bool {
    fn exec(self,
            original_self: &'largs ::opengl_framebuffer_object::OpenGLFramebufferObject)
            -> ::cpp_utils::CppBox<::image::Image> {
      let flipped = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_toImage_as_ptr_flipped(original_self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject, flipped) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> OpenGLFramebufferObjectToImageArgs<'largs> for (bool, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::opengl_framebuffer_object::OpenGLFramebufferObject)
            -> ::cpp_utils::CppBox<::image::Image> {
      let flipped = self.0;
      let color_attachment_index = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_toImage_as_ptr_flipped_colorAttachmentIndex(original_self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject, flipped, color_attachment_index) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> OpenGLFramebufferObjectToImageArgs<'largs> for () {
    fn exec(self,
            original_self: &'largs ::opengl_framebuffer_object::OpenGLFramebufferObject)
            -> ::cpp_utils::CppBox<::image::Image> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFramebufferObject_toImage_as_ptr_no_args(original_self as *const ::opengl_framebuffer_object::OpenGLFramebufferObject) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
