/// C++ type: <span style='color: green;'>```QOpenGLBuffer::Access```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Access {
  /// C++ enum variant: <span style='color: green;'>```ReadOnly = 35000```</span>
  ReadOnly = 35000,
  /// C++ enum variant: <span style='color: green;'>```WriteOnly = 35001```</span>
  WriteOnly = 35001,
  /// C++ enum variant: <span style='color: green;'>```ReadWrite = 35002```</span>
  ReadWrite = 35002,
}

/// C++ type: <span style='color: green;'>```QOpenGLBuffer```</span>
#[repr(C)]
pub struct OpenGLBuffer([u8; ::type_sizes::QT_GUI_OPENGL_BUFFER_OPEN_G_L_BUFFER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for OpenGLBuffer {
  unsafe fn new_uninitialized() -> OpenGLBuffer {
    OpenGLBuffer(::std::mem::uninitialized())
  }
}

impl OpenGLBuffer {
  /// C++ method: <span style='color: green;'>```void QOpenGLBuffer::allocate(int count)```</span>
  ///
  ///
  pub fn allocate(&mut self, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_allocate_count(self as *mut ::opengl_buffer::OpenGLBuffer, count) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLBuffer::allocate(const void* data, int count)```</span>
  ///
  ///
  pub unsafe fn allocate_unsafe(&mut self, data: *const ::libc::c_void, count: ::libc::c_int) {
    ::ffi::qt_gui_c_QOpenGLBuffer_allocate_data_count(self as *mut ::opengl_buffer::OpenGLBuffer, data, count)
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLBuffer::bind()```</span>
  ///
  ///
  pub fn bind(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_bind(self as *mut ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLBuffer::bufferId() const```</span>
  ///
  ///
  pub fn buffer_id(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_bufferId(self as *const ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLBuffer::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_create(self as *mut ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLBuffer::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_destroy(self as *mut ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLBuffer::isCreated() const```</span>
  ///
  ///
  pub fn is_created(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_isCreated(self as *const ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```void* QOpenGLBuffer::map(QOpenGLBuffer::Access access)```</span>
  ///
  ///
  pub fn map(&mut self, access: &::opengl_buffer::Access) -> *mut ::libc::c_void {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLBuffer_map(self as *mut ::opengl_buffer::OpenGLBuffer,
                                        access as *const ::opengl_buffer::Access)
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLBuffer::QOpenGLBuffer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::opengl_buffer::OpenGLBuffer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLBuffer::QOpenGLBuffer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::opengl_buffer::Type) -> ::opengl_buffer::OpenGLBuffer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLBuffer::QOpenGLBuffer(QOpenGLBuffer::Type type)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::opengl_buffer::OpenGLBuffer) -> ::opengl_buffer::OpenGLBuffer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLBuffer::QOpenGLBuffer(const QOpenGLBuffer& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::opengl_buffer::OpenGLBuffer
    where Args: overloading::OpenGLBufferNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLBuffer& QOpenGLBuffer::operator=(const QOpenGLBuffer& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::opengl_buffer::OpenGLBuffer)
                             -> &'l0 mut ::opengl_buffer::OpenGLBuffer {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QOpenGLBuffer_operator_assign(self as *mut ::opengl_buffer::OpenGLBuffer,
                                                    other as *const ::opengl_buffer::OpenGLBuffer)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLBuffer::read(int offset, void* data, int count)```</span>
  ///
  ///
  pub unsafe fn read(&mut self, offset: ::libc::c_int, data: *mut ::libc::c_void, count: ::libc::c_int) -> bool {
    ::ffi::qt_gui_c_QOpenGLBuffer_read(self as *mut ::opengl_buffer::OpenGLBuffer,
                                       offset,
                                       data,
                                       count)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLBuffer::release()```</span>
  ///
  ///
  pub fn release(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_release_no_args(self as *mut ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```static void QOpenGLBuffer::release(QOpenGLBuffer::Type type)```</span>
  ///
  ///
  pub fn release_static(type_: &::opengl_buffer::Type) {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_release_type(type_ as *const ::opengl_buffer::Type) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLBuffer::setUsagePattern(QOpenGLBuffer::UsagePattern value)```</span>
  ///
  ///
  pub fn set_usage_pattern(&mut self, value: &::opengl_buffer::UsagePattern) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLBuffer_setUsagePattern(self as *mut ::opengl_buffer::OpenGLBuffer,
                                                    value as *const ::opengl_buffer::UsagePattern)
    }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLBuffer::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_size(self as *const ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLBuffer::unmap()```</span>
  ///
  ///
  pub fn unmap(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_unmap(self as *mut ::opengl_buffer::OpenGLBuffer) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLBuffer::write(int offset, const void* data, int count)```</span>
  ///
  ///
  pub unsafe fn write(&mut self, offset: ::libc::c_int, data: *const ::libc::c_void, count: ::libc::c_int) {
    ::ffi::qt_gui_c_QOpenGLBuffer_write(self as *mut ::opengl_buffer::OpenGLBuffer,
                                        offset,
                                        data,
                                        count)
  }
}

impl Drop for ::opengl_buffer::OpenGLBuffer {
  /// C++ method: <span style='color: green;'>```[destructor] void QOpenGLBuffer::~QOpenGLBuffer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLBuffer_destructor(self as *mut ::opengl_buffer::OpenGLBuffer) }
  }
}

/// C++ type: <span style='color: green;'>```QOpenGLBuffer::RangeAccessFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RangeAccessFlag {
  /// C++ enum variant: <span style='color: green;'>```RangeRead = 1```</span>
  Read = 1,
  /// C++ enum variant: <span style='color: green;'>```RangeWrite = 2```</span>
  Write = 2,
  /// C++ enum variant: <span style='color: green;'>```RangeInvalidate = 4```</span>
  Invalidate = 4,
  /// C++ enum variant: <span style='color: green;'>```RangeInvalidateBuffer = 8```</span>
  InvalidateBuffer = 8,
  /// C++ enum variant: <span style='color: green;'>```RangeFlushExplicit = 16```</span>
  FlushExplicit = 16,
  /// C++ enum variant: <span style='color: green;'>```RangeUnsynchronized = 32```</span>
  Unsynchronized = 32,
}

/// C++ type: <span style='color: green;'>```QOpenGLBuffer::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```VertexBuffer = 34962```</span>
  Vertex = 34962,
  /// C++ enum variant: <span style='color: green;'>```IndexBuffer = 34963```</span>
  Index = 34963,
  /// C++ enum variant: <span style='color: green;'>```PixelPackBuffer = 35051```</span>
  PixelPack = 35051,
  /// C++ enum variant: <span style='color: green;'>```PixelUnpackBuffer = 35052```</span>
  PixelUnpack = 35052,
}

/// C++ type: <span style='color: green;'>```QOpenGLBuffer::UsagePattern```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UsagePattern {
  /// C++ enum variant: <span style='color: green;'>```StreamDraw = 35040```</span>
  StreamDraw = 35040,
  /// C++ enum variant: <span style='color: green;'>```StreamRead = 35041```</span>
  StreamRead = 35041,
  /// C++ enum variant: <span style='color: green;'>```StreamCopy = 35042```</span>
  StreamCopy = 35042,
  /// C++ enum variant: <span style='color: green;'>```StaticDraw = 35044```</span>
  StaticDraw = 35044,
  /// C++ enum variant: <span style='color: green;'>```StaticRead = 35045```</span>
  StaticRead = 35045,
  /// C++ enum variant: <span style='color: green;'>```StaticCopy = 35046```</span>
  StaticCopy = 35046,
  /// C++ enum variant: <span style='color: green;'>```DynamicDraw = 35048```</span>
  DynamicDraw = 35048,
  /// C++ enum variant: <span style='color: green;'>```DynamicRead = 35049```</span>
  DynamicRead = 35049,
  /// C++ enum variant: <span style='color: green;'>```DynamicCopy = 35050```</span>
  DynamicCopy = 35050,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLBuffer::new](../struct.OpenGLBuffer.html#method.new) method.
  pub trait OpenGLBufferNewArgs {
    fn exec(self) -> ::opengl_buffer::OpenGLBuffer;
  }
  impl OpenGLBufferNewArgs for () {
    fn exec(self) -> ::opengl_buffer::OpenGLBuffer {

      {
        let mut object: ::opengl_buffer::OpenGLBuffer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLBuffer_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLBufferNewArgs for &'a ::opengl_buffer::OpenGLBuffer {
    fn exec(self) -> ::opengl_buffer::OpenGLBuffer {
      let other = self;
      {
        let mut object: ::opengl_buffer::OpenGLBuffer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLBuffer_constructor_other(other as *const ::opengl_buffer::OpenGLBuffer, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLBufferNewArgs for &'a ::opengl_buffer::Type {
    fn exec(self) -> ::opengl_buffer::OpenGLBuffer {
      let type_ = self;
      {
        let mut object: ::opengl_buffer::OpenGLBuffer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLBuffer_constructor_type(type_ as *const ::opengl_buffer::Type, &mut object);
        }
        object
      }
    }
  }
}
