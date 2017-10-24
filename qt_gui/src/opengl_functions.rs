/// C++ type: <span style='color: green;'>```QOpenGLFunctions::OpenGLFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OpenGLFeature {
  /// C++ enum variant: <span style='color: green;'>```Multitexture = 1```</span>
  Multitexture = 1,
  /// C++ enum variant: <span style='color: green;'>```Shaders = 2```</span>
  Shaders = 2,
  /// C++ enum variant: <span style='color: green;'>```Buffers = 4```</span>
  Buffers = 4,
  /// C++ enum variant: <span style='color: green;'>```Framebuffers = 8```</span>
  Framebuffers = 8,
  /// C++ enum variant: <span style='color: green;'>```BlendColor = 16```</span>
  BlendColor = 16,
  /// C++ enum variant: <span style='color: green;'>```BlendEquation = 32```</span>
  BlendEquation = 32,
  /// C++ enum variant: <span style='color: green;'>```BlendEquationSeparate = 64```</span>
  BlendEquationSeparate = 64,
  /// C++ enum variant: <span style='color: green;'>```BlendFuncSeparate = 128```</span>
  BlendFuncSeparate = 128,
  /// C++ enum variant: <span style='color: green;'>```BlendSubtract = 256```</span>
  BlendSubtract = 256,
  /// C++ enum variant: <span style='color: green;'>```CompressedTextures = 512```</span>
  CompressedTextures = 512,
  /// C++ enum variant: <span style='color: green;'>```Multisample = 1024```</span>
  Multisample = 1024,
  /// C++ enum variant: <span style='color: green;'>```StencilSeparate = 2048```</span>
  StencilSeparate = 2048,
  /// C++ enum variant: <span style='color: green;'>```NPOTTextures = 4096```</span>
  NPOTTextures = 4096,
  /// C++ enum variant: <span style='color: green;'>```NPOTTextureRepeat = 8192```</span>
  NPOTTextureRepeat = 8192,
  /// C++ enum variant: <span style='color: green;'>```FixedFunctionPipeline = 16384```</span>
  FixedFunctionPipeline = 16384,
  /// C++ enum variant: <span style='color: green;'>```TextureRGFormats = 32768```</span>
  TextureRGFormats = 32768,
  /// C++ enum variant: <span style='color: green;'>```MultipleRenderTargets = 65536```</span>
  MultipleRenderTargets = 65536,
}

/// C++ type: <span style='color: green;'>```QOpenGLFunctions```</span>
#[repr(C)]
pub struct OpenGLFunctions(u8);

impl OpenGLFunctions {
  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glActiveTexture(unsigned int texture)```</span>
  ///
  ///
  pub fn gl_active_texture(&mut self, texture: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glActiveTexture(self as *mut ::opengl_functions::OpenGLFunctions, texture)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glAttachShader(GLuint program, GLuint shader)```</span>
  ///
  ///
  pub fn gl_attach_shader(&mut self, program: u32, shader: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glAttachShader(self as *mut ::opengl_functions::OpenGLFunctions,
                                                      program,
                                                      shader)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBindAttribLocation(GLuint program, GLuint index, const char* name)```</span>
  ///
  ///
  pub unsafe fn gl_bind_attrib_location(&mut self, program: u32, index: u32, name: *const ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glBindAttribLocation(self as *mut ::opengl_functions::OpenGLFunctions,
                                                          program,
                                                          index,
                                                          name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBindBuffer(unsigned int target, GLuint buffer)```</span>
  ///
  ///
  pub fn gl_bind_buffer(&mut self, target: ::libc::c_uint, buffer: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBindBuffer(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    target,
                                                    buffer)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBindFramebuffer(unsigned int target, GLuint framebuffer)```</span>
  ///
  ///
  pub fn gl_bind_framebuffer(&mut self, target: ::libc::c_uint, framebuffer: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBindFramebuffer(self as *mut ::opengl_functions::OpenGLFunctions,
                                                         target,
                                                         framebuffer)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBindRenderbuffer(unsigned int target, GLuint renderbuffer)```</span>
  ///
  ///
  pub fn gl_bind_renderbuffer(&mut self, target: ::libc::c_uint, renderbuffer: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBindRenderbuffer(self as *mut ::opengl_functions::OpenGLFunctions,
                                                          target,
                                                          renderbuffer)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBindTexture(unsigned int target, GLuint texture)```</span>
  ///
  ///
  pub fn gl_bind_texture(&mut self, target: ::libc::c_uint, texture: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBindTexture(self as *mut ::opengl_functions::OpenGLFunctions,
                                                     target,
                                                     texture)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBlendColor(float red, float green, float blue, float alpha)```</span>
  ///
  ///
  pub fn gl_blend_color(&mut self,
                        red: ::libc::c_float,
                        green: ::libc::c_float,
                        blue: ::libc::c_float,
                        alpha: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBlendColor(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    red,
                                                    green,
                                                    blue,
                                                    alpha)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBlendEquation(unsigned int mode)```</span>
  ///
  ///
  pub fn gl_blend_equation(&mut self, mode: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glBlendEquation(self as *mut ::opengl_functions::OpenGLFunctions, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBlendEquationSeparate(unsigned int modeRGB, unsigned int modeAlpha)```</span>
  ///
  ///
  pub fn gl_blend_equation_separate(&mut self, mode_r_g_b: ::libc::c_uint, mode_alpha: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBlendEquationSeparate(self as *mut ::opengl_functions::OpenGLFunctions,
                                                               mode_r_g_b,
                                                               mode_alpha)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBlendFunc(unsigned int sfactor, unsigned int dfactor)```</span>
  ///
  ///
  pub fn gl_blend_func(&mut self, sfactor: ::libc::c_uint, dfactor: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBlendFunc(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   sfactor,
                                                   dfactor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBlendFuncSeparate(unsigned int srcRGB, unsigned int dstRGB, unsigned int srcAlpha, unsigned int dstAlpha)```</span>
  ///
  ///
  pub fn gl_blend_func_separate(&mut self,
                                src_r_g_b: ::libc::c_uint,
                                dst_r_g_b: ::libc::c_uint,
                                src_alpha: ::libc::c_uint,
                                dst_alpha: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glBlendFuncSeparate(self as *mut ::opengl_functions::OpenGLFunctions,
                                                           src_r_g_b,
                                                           dst_r_g_b,
                                                           src_alpha,
                                                           dst_alpha)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBufferData(unsigned int target, long size, const void* data, unsigned int usage)```</span>
  ///
  ///
  pub unsafe fn gl_buffer_data(&mut self,
                               target: ::libc::c_uint,
                               size: ::libc::c_long,
                               data: *const ::libc::c_void,
                               usage: ::libc::c_uint) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glBufferData(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  target,
                                                  size,
                                                  data,
                                                  usage)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glBufferSubData(unsigned int target, long offset, long size, const void* data)```</span>
  ///
  ///
  pub unsafe fn gl_buffer_sub_data(&mut self,
                                   target: ::libc::c_uint,
                                   offset: ::libc::c_long,
                                   size: ::libc::c_long,
                                   data: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glBufferSubData(self as *mut ::opengl_functions::OpenGLFunctions,
                                                     target,
                                                     offset,
                                                     size,
                                                     data)
  }

  /// C++ method: <span style='color: green;'>```unsigned int QOpenGLFunctions::glCheckFramebufferStatus(unsigned int target)```</span>
  ///
  ///
  pub fn gl_check_framebuffer_status(&mut self, target: ::libc::c_uint) -> ::libc::c_uint {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glCheckFramebufferStatus(self as *mut ::opengl_functions::OpenGLFunctions,
                                                                target)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glClear(unsigned int mask)```</span>
  ///
  ///
  pub fn gl_clear(&mut self, mask: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glClear(self as *mut ::opengl_functions::OpenGLFunctions, mask) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glClearColor(float red, float green, float blue, float alpha)```</span>
  ///
  ///
  pub fn gl_clear_color(&mut self,
                        red: ::libc::c_float,
                        green: ::libc::c_float,
                        blue: ::libc::c_float,
                        alpha: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glClearColor(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    red,
                                                    green,
                                                    blue,
                                                    alpha)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glClearDepthf(float depth)```</span>
  ///
  ///
  pub fn gl_clear_depthf(&mut self, depth: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glClearDepthf(self as *mut ::opengl_functions::OpenGLFunctions, depth) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glClearStencil(GLint s)```</span>
  ///
  ///
  pub fn gl_clear_stencil(&mut self, s: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glClearStencil(self as *mut ::opengl_functions::OpenGLFunctions, s) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glColorMask(unsigned char red, unsigned char green, unsigned char blue, unsigned char alpha)```</span>
  ///
  ///
  pub fn gl_color_mask(&mut self,
                       red: ::libc::c_uchar,
                       green: ::libc::c_uchar,
                       blue: ::libc::c_uchar,
                       alpha: ::libc::c_uchar) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glColorMask(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   red,
                                                   green,
                                                   blue,
                                                   alpha)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glCompileShader(GLuint shader)```</span>
  ///
  ///
  pub fn gl_compile_shader(&mut self, shader: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glCompileShader(self as *mut ::opengl_functions::OpenGLFunctions, shader)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glCompressedTexImage2D(unsigned int target, GLint level, unsigned int internalformat, int width, int height, GLint border, int imageSize, const void* data)```</span>
  ///
  ///
  pub unsafe fn gl_compressed_tex_image_2d(&mut self,
                                           target: ::libc::c_uint,
                                           level: i32,
                                           internalformat: ::libc::c_uint,
                                           width: ::libc::c_int,
                                           height: ::libc::c_int,
                                           border: i32,
                                           image_size: ::libc::c_int,
                                           data: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glCompressedTexImage2D(self as *mut ::opengl_functions::OpenGLFunctions,
                                                            target,
                                                            level,
                                                            internalformat,
                                                            width,
                                                            height,
                                                            border,
                                                            image_size,
                                                            data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glCompressedTexSubImage2D(unsigned int target, GLint level, GLint xoffset, GLint yoffset, int width, int height, unsigned int format, int imageSize, const void* data)```</span>
  ///
  ///
  pub unsafe fn gl_compressed_tex_sub_image_2d(&mut self,
                                               target: ::libc::c_uint,
                                               level: i32,
                                               xoffset: i32,
                                               yoffset: i32,
                                               width: ::libc::c_int,
                                               height: ::libc::c_int,
                                               format: ::libc::c_uint,
                                               image_size: ::libc::c_int,
                                               data: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glCompressedTexSubImage2D(self as *mut ::opengl_functions::OpenGLFunctions,
                                                               target,
                                                               level,
                                                               xoffset,
                                                               yoffset,
                                                               width,
                                                               height,
                                                               format,
                                                               image_size,
                                                               data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glCopyTexImage2D(unsigned int target, GLint level, unsigned int internalformat, GLint x, GLint y, int width, int height, GLint border)```</span>
  ///
  ///
  pub fn gl_copy_tex_image_2d(&mut self,
                              target: ::libc::c_uint,
                              level: i32,
                              internalformat: ::libc::c_uint,
                              x: i32,
                              y: i32,
                              width: ::libc::c_int,
                              height: ::libc::c_int,
                              border: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glCopyTexImage2D(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        target,
                                                        level,
                                                        internalformat,
                                                        x,
                                                        y,
                                                        width,
                                                        height,
                                                        border)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glCopyTexSubImage2D(unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, int width, int height)```</span>
  ///
  ///
  pub fn gl_copy_tex_sub_image_2d(&mut self,
                                  target: ::libc::c_uint,
                                  level: i32,
                                  xoffset: i32,
                                  yoffset: i32,
                                  x: i32,
                                  y: i32,
                                  width: ::libc::c_int,
                                  height: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glCopyTexSubImage2D(self as *mut ::opengl_functions::OpenGLFunctions,
                                                           target,
                                                           level,
                                                           xoffset,
                                                           yoffset,
                                                           x,
                                                           y,
                                                           width,
                                                           height)
    }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLFunctions::glCreateProgram()```</span>
  ///
  ///
  pub fn gl_create_program(&mut self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glCreateProgram(self as *mut ::opengl_functions::OpenGLFunctions) }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLFunctions::glCreateShader(unsigned int type)```</span>
  ///
  ///
  pub fn gl_create_shader(&mut self, type_: ::libc::c_uint) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glCreateShader(self as *mut ::opengl_functions::OpenGLFunctions, type_) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glCullFace(unsigned int mode)```</span>
  ///
  ///
  pub fn gl_cull_face(&mut self, mode: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glCullFace(self as *mut ::opengl_functions::OpenGLFunctions, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDeleteBuffers(int n, const GLuint* buffers)```</span>
  ///
  ///
  pub unsafe fn gl_delete_buffers(&mut self, n: ::libc::c_int, buffers: *const u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glDeleteBuffers(self as *mut ::opengl_functions::OpenGLFunctions, n, buffers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDeleteFramebuffers(int n, const GLuint* framebuffers)```</span>
  ///
  ///
  pub unsafe fn gl_delete_framebuffers(&mut self, n: ::libc::c_int, framebuffers: *const u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glDeleteFramebuffers(self as *mut ::opengl_functions::OpenGLFunctions,
                                                          n,
                                                          framebuffers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDeleteProgram(GLuint program)```</span>
  ///
  ///
  pub fn gl_delete_program(&mut self, program: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glDeleteProgram(self as *mut ::opengl_functions::OpenGLFunctions, program)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDeleteRenderbuffers(int n, const GLuint* renderbuffers)```</span>
  ///
  ///
  pub unsafe fn gl_delete_renderbuffers(&mut self, n: ::libc::c_int, renderbuffers: *const u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glDeleteRenderbuffers(self as *mut ::opengl_functions::OpenGLFunctions,
                                                           n,
                                                           renderbuffers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDeleteShader(GLuint shader)```</span>
  ///
  ///
  pub fn gl_delete_shader(&mut self, shader: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glDeleteShader(self as *mut ::opengl_functions::OpenGLFunctions, shader) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDeleteTextures(int n, const GLuint* textures)```</span>
  ///
  ///
  pub unsafe fn gl_delete_textures(&mut self, n: ::libc::c_int, textures: *const u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glDeleteTextures(self as *mut ::opengl_functions::OpenGLFunctions,
                                                      n,
                                                      textures)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDepthFunc(unsigned int func)```</span>
  ///
  ///
  pub fn gl_depth_func(&mut self, func: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glDepthFunc(self as *mut ::opengl_functions::OpenGLFunctions, func) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDepthMask(unsigned char flag)```</span>
  ///
  ///
  pub fn gl_depth_mask(&mut self, flag: ::libc::c_uchar) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glDepthMask(self as *mut ::opengl_functions::OpenGLFunctions, flag) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDepthRangef(float zNear, float zFar)```</span>
  ///
  ///
  pub fn gl_depth_rangef(&mut self, z_near: ::libc::c_float, z_far: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glDepthRangef(self as *mut ::opengl_functions::OpenGLFunctions,
                                                     z_near,
                                                     z_far)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDetachShader(GLuint program, GLuint shader)```</span>
  ///
  ///
  pub fn gl_detach_shader(&mut self, program: u32, shader: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glDetachShader(self as *mut ::opengl_functions::OpenGLFunctions,
                                                      program,
                                                      shader)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDisable(unsigned int cap)```</span>
  ///
  ///
  pub fn gl_disable(&mut self, cap: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glDisable(self as *mut ::opengl_functions::OpenGLFunctions, cap) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDisableVertexAttribArray(GLuint index)```</span>
  ///
  ///
  pub fn gl_disable_vertex_attrib_array(&mut self, index: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glDisableVertexAttribArray(self as *mut ::opengl_functions::OpenGLFunctions,
                                                                  index)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDrawArrays(unsigned int mode, GLint first, int count)```</span>
  ///
  ///
  pub fn gl_draw_arrays(&mut self, mode: ::libc::c_uint, first: i32, count: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glDrawArrays(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    mode,
                                                    first,
                                                    count)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glDrawElements(unsigned int mode, int count, unsigned int type, const void* indices)```</span>
  ///
  ///
  pub unsafe fn gl_draw_elements(&mut self,
                                 mode: ::libc::c_uint,
                                 count: ::libc::c_int,
                                 type_: ::libc::c_uint,
                                 indices: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glDrawElements(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    mode,
                                                    count,
                                                    type_,
                                                    indices)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glEnable(unsigned int cap)```</span>
  ///
  ///
  pub fn gl_enable(&mut self, cap: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glEnable(self as *mut ::opengl_functions::OpenGLFunctions, cap) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glEnableVertexAttribArray(GLuint index)```</span>
  ///
  ///
  pub fn gl_enable_vertex_attrib_array(&mut self, index: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glEnableVertexAttribArray(self as *mut ::opengl_functions::OpenGLFunctions,
                                                                 index)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glFinish()```</span>
  ///
  ///
  pub fn gl_finish(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glFinish(self as *mut ::opengl_functions::OpenGLFunctions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glFlush()```</span>
  ///
  ///
  pub fn gl_flush(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glFlush(self as *mut ::opengl_functions::OpenGLFunctions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glFramebufferRenderbuffer(unsigned int target, unsigned int attachment, unsigned int renderbuffertarget, GLuint renderbuffer)```</span>
  ///
  ///
  pub fn gl_framebuffer_renderbuffer(&mut self,
                                     target: ::libc::c_uint,
                                     attachment: ::libc::c_uint,
                                     renderbuffertarget: ::libc::c_uint,
                                     renderbuffer: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glFramebufferRenderbuffer(self as *mut ::opengl_functions::OpenGLFunctions,
                                                                 target,
                                                                 attachment,
                                                                 renderbuffertarget,
                                                                 renderbuffer)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glFramebufferTexture2D(unsigned int target, unsigned int attachment, unsigned int textarget, GLuint texture, GLint level)```</span>
  ///
  ///
  pub fn gl_framebuffer_texture_2d(&mut self,
                                   target: ::libc::c_uint,
                                   attachment: ::libc::c_uint,
                                   textarget: ::libc::c_uint,
                                   texture: u32,
                                   level: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glFramebufferTexture2D(self as *mut ::opengl_functions::OpenGLFunctions,
                                                              target,
                                                              attachment,
                                                              textarget,
                                                              texture,
                                                              level)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glFrontFace(unsigned int mode)```</span>
  ///
  ///
  pub fn gl_front_face(&mut self, mode: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glFrontFace(self as *mut ::opengl_functions::OpenGLFunctions, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGenBuffers(int n, GLuint* buffers)```</span>
  ///
  ///
  pub unsafe fn gl_gen_buffers(&mut self, n: ::libc::c_int, buffers: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGenBuffers(self as *mut ::opengl_functions::OpenGLFunctions, n, buffers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGenFramebuffers(int n, GLuint* framebuffers)```</span>
  ///
  ///
  pub unsafe fn gl_gen_framebuffers(&mut self, n: ::libc::c_int, framebuffers: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGenFramebuffers(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       n,
                                                       framebuffers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGenRenderbuffers(int n, GLuint* renderbuffers)```</span>
  ///
  ///
  pub unsafe fn gl_gen_renderbuffers(&mut self, n: ::libc::c_int, renderbuffers: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGenRenderbuffers(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        n,
                                                        renderbuffers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGenTextures(int n, GLuint* textures)```</span>
  ///
  ///
  pub unsafe fn gl_gen_textures(&mut self, n: ::libc::c_int, textures: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGenTextures(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   n,
                                                   textures)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGenerateMipmap(unsigned int target)```</span>
  ///
  ///
  pub fn gl_generate_mipmap(&mut self, target: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glGenerateMipmap(self as *mut ::opengl_functions::OpenGLFunctions, target)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetActiveAttrib(GLuint program, GLuint index, int bufsize, int* length, GLint* size, unsigned int* type, char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_active_attrib(&mut self,
                                     program: u32,
                                     index: u32,
                                     bufsize: ::libc::c_int,
                                     length: *mut ::libc::c_int,
                                     size: *mut i32,
                                     type_: *mut ::libc::c_uint,
                                     name: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetActiveAttrib(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       program,
                                                       index,
                                                       bufsize,
                                                       length,
                                                       size,
                                                       type_,
                                                       name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetActiveUniform(GLuint program, GLuint index, int bufsize, int* length, GLint* size, unsigned int* type, char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_active_uniform(&mut self,
                                      program: u32,
                                      index: u32,
                                      bufsize: ::libc::c_int,
                                      length: *mut ::libc::c_int,
                                      size: *mut i32,
                                      type_: *mut ::libc::c_uint,
                                      name: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetActiveUniform(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        program,
                                                        index,
                                                        bufsize,
                                                        length,
                                                        size,
                                                        type_,
                                                        name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetAttachedShaders(GLuint program, int maxcount, int* count, GLuint* shaders)```</span>
  ///
  ///
  pub unsafe fn gl_get_attached_shaders(&mut self,
                                        program: u32,
                                        maxcount: ::libc::c_int,
                                        count: *mut ::libc::c_int,
                                        shaders: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetAttachedShaders(self as *mut ::opengl_functions::OpenGLFunctions,
                                                          program,
                                                          maxcount,
                                                          count,
                                                          shaders)
  }

  /// C++ method: <span style='color: green;'>```GLint QOpenGLFunctions::glGetAttribLocation(GLuint program, const char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_attrib_location(&mut self, program: u32, name: *const ::libc::c_char) -> i32 {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetAttribLocation(self as *mut ::opengl_functions::OpenGLFunctions,
                                                         program,
                                                         name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetBooleanv(unsigned int pname, unsigned char* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_booleanv(&mut self, pname: ::libc::c_uint, params: *mut ::libc::c_uchar) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetBooleanv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   pname,
                                                   params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetBufferParameteriv(unsigned int target, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_buffer_parameteriv(&mut self, target: ::libc::c_uint, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetBufferParameteriv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                            target,
                                                            pname,
                                                            params)
  }

  /// C++ method: <span style='color: green;'>```unsigned int QOpenGLFunctions::glGetError()```</span>
  ///
  ///
  pub fn gl_get_error(&mut self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glGetError(self as *mut ::opengl_functions::OpenGLFunctions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetFloatv(unsigned int pname, float* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_floatv(&mut self, pname: ::libc::c_uint, params: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetFloatv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                 pname,
                                                 params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetFramebufferAttachmentParameteriv(unsigned int target, unsigned int attachment, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_framebuffer_attachment_parameteriv(&mut self,
                                                          target: ::libc::c_uint,
                                                          attachment: ::libc::c_uint,
                                                          pname: ::libc::c_uint,
                                                          params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetFramebufferAttachmentParameteriv(self as *mut ::opengl_functions::OpenGLFunctions, target, attachment, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetIntegerv(unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_integerv(&mut self, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetIntegerv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   pname,
                                                   params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetProgramInfoLog(GLuint program, int bufsize, int* length, char* infolog)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_info_log(&mut self,
                                        program: u32,
                                        bufsize: ::libc::c_int,
                                        length: *mut ::libc::c_int,
                                        infolog: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetProgramInfoLog(self as *mut ::opengl_functions::OpenGLFunctions,
                                                         program,
                                                         bufsize,
                                                         length,
                                                         infolog)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetProgramiv(GLuint program, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_programiv(&mut self, program: u32, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetProgramiv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    program,
                                                    pname,
                                                    params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetRenderbufferParameteriv(unsigned int target, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_renderbuffer_parameteriv(&mut self,
                                                target: ::libc::c_uint,
                                                pname: ::libc::c_uint,
                                                params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetRenderbufferParameteriv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                                  target,
                                                                  pname,
                                                                  params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetShaderInfoLog(GLuint shader, int bufsize, int* length, char* infolog)```</span>
  ///
  ///
  pub unsafe fn gl_get_shader_info_log(&mut self,
                                       shader: u32,
                                       bufsize: ::libc::c_int,
                                       length: *mut ::libc::c_int,
                                       infolog: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetShaderInfoLog(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        shader,
                                                        bufsize,
                                                        length,
                                                        infolog)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetShaderPrecisionFormat(unsigned int shadertype, unsigned int precisiontype, GLint* range, GLint* precision)```</span>
  ///
  ///
  pub unsafe fn gl_get_shader_precision_format(&mut self,
                                               shadertype: ::libc::c_uint,
                                               precisiontype: ::libc::c_uint,
                                               range: *mut i32,
                                               precision: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetShaderPrecisionFormat(self as *mut ::opengl_functions::OpenGLFunctions,
                                                                shadertype,
                                                                precisiontype,
                                                                range,
                                                                precision)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetShaderSource(GLuint shader, int bufsize, int* length, char* source)```</span>
  ///
  ///
  pub unsafe fn gl_get_shader_source(&mut self,
                                     shader: u32,
                                     bufsize: ::libc::c_int,
                                     length: *mut ::libc::c_int,
                                     source: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetShaderSource(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       shader,
                                                       bufsize,
                                                       length,
                                                       source)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetShaderiv(GLuint shader, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_shaderiv(&mut self, shader: u32, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetShaderiv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   shader,
                                                   pname,
                                                   params)
  }

  /// C++ method: <span style='color: green;'>```const GLubyte* QOpenGLFunctions::glGetString(unsigned int name)```</span>
  ///
  ///
  pub fn gl_get_string(&mut self, name: ::libc::c_uint) -> *const u8 {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glGetString(self as *mut ::opengl_functions::OpenGLFunctions, name) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetTexParameterfv(unsigned int target, unsigned int pname, float* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_tex_parameterfv(&mut self,
                                       target: ::libc::c_uint,
                                       pname: ::libc::c_uint,
                                       params: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetTexParameterfv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                         target,
                                                         pname,
                                                         params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetTexParameteriv(unsigned int target, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_tex_parameteriv(&mut self, target: ::libc::c_uint, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetTexParameteriv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                         target,
                                                         pname,
                                                         params)
  }

  /// C++ method: <span style='color: green;'>```GLint QOpenGLFunctions::glGetUniformLocation(GLuint program, const char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_uniform_location(&mut self, program: u32, name: *const ::libc::c_char) -> i32 {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetUniformLocation(self as *mut ::opengl_functions::OpenGLFunctions,
                                                          program,
                                                          name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetUniformfv(GLuint program, GLint location, float* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_uniformfv(&mut self, program: u32, location: i32, params: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetUniformfv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    program,
                                                    location,
                                                    params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetUniformiv(GLuint program, GLint location, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_uniformiv(&mut self, program: u32, location: i32, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetUniformiv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    program,
                                                    location,
                                                    params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetVertexAttribPointerv(GLuint index, unsigned int pname, void** pointer)```</span>
  ///
  ///
  pub unsafe fn gl_get_vertex_attrib_pointerv(&mut self,
                                              index: u32,
                                              pname: ::libc::c_uint,
                                              pointer: *mut *mut ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetVertexAttribPointerv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                               index,
                                                               pname,
                                                               pointer)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetVertexAttribfv(GLuint index, unsigned int pname, float* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_vertex_attribfv(&mut self, index: u32, pname: ::libc::c_uint, params: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetVertexAttribfv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                         index,
                                                         pname,
                                                         params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glGetVertexAttribiv(GLuint index, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_vertex_attribiv(&mut self, index: u32, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glGetVertexAttribiv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                         index,
                                                         pname,
                                                         params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glHint(unsigned int target, unsigned int mode)```</span>
  ///
  ///
  pub fn gl_hint(&mut self, target: ::libc::c_uint, mode: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glHint(self as *mut ::opengl_functions::OpenGLFunctions,
                                              target,
                                              mode)
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLFunctions::glIsBuffer(GLuint buffer)```</span>
  ///
  ///
  pub fn gl_is_buffer(&mut self, buffer: u32) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glIsBuffer(self as *mut ::opengl_functions::OpenGLFunctions, buffer) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLFunctions::glIsEnabled(unsigned int cap)```</span>
  ///
  ///
  pub fn gl_is_enabled(&mut self, cap: ::libc::c_uint) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glIsEnabled(self as *mut ::opengl_functions::OpenGLFunctions, cap) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLFunctions::glIsFramebuffer(GLuint framebuffer)```</span>
  ///
  ///
  pub fn gl_is_framebuffer(&mut self, framebuffer: u32) -> ::libc::c_uchar {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glIsFramebuffer(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       framebuffer)
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLFunctions::glIsProgram(GLuint program)```</span>
  ///
  ///
  pub fn gl_is_program(&mut self, program: u32) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glIsProgram(self as *mut ::opengl_functions::OpenGLFunctions, program) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLFunctions::glIsRenderbuffer(GLuint renderbuffer)```</span>
  ///
  ///
  pub fn gl_is_renderbuffer(&mut self, renderbuffer: u32) -> ::libc::c_uchar {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glIsRenderbuffer(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        renderbuffer)
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLFunctions::glIsShader(GLuint shader)```</span>
  ///
  ///
  pub fn gl_is_shader(&mut self, shader: u32) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glIsShader(self as *mut ::opengl_functions::OpenGLFunctions, shader) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLFunctions::glIsTexture(GLuint texture)```</span>
  ///
  ///
  pub fn gl_is_texture(&mut self, texture: u32) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glIsTexture(self as *mut ::opengl_functions::OpenGLFunctions, texture) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glLineWidth(float width)```</span>
  ///
  ///
  pub fn gl_line_width(&mut self, width: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glLineWidth(self as *mut ::opengl_functions::OpenGLFunctions, width) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glLinkProgram(GLuint program)```</span>
  ///
  ///
  pub fn gl_link_program(&mut self, program: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glLinkProgram(self as *mut ::opengl_functions::OpenGLFunctions, program) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glPixelStorei(unsigned int pname, GLint param)```</span>
  ///
  ///
  pub fn gl_pixel_storei(&mut self, pname: ::libc::c_uint, param: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glPixelStorei(self as *mut ::opengl_functions::OpenGLFunctions,
                                                     pname,
                                                     param)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glPolygonOffset(float factor, float units)```</span>
  ///
  ///
  pub fn gl_polygon_offset(&mut self, factor: ::libc::c_float, units: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glPolygonOffset(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       factor,
                                                       units)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glReadPixels(GLint x, GLint y, int width, int height, unsigned int format, unsigned int type, void* pixels)```</span>
  ///
  ///
  pub unsafe fn gl_read_pixels(&mut self,
                               x: i32,
                               y: i32,
                               width: ::libc::c_int,
                               height: ::libc::c_int,
                               format: ::libc::c_uint,
                               type_: ::libc::c_uint,
                               pixels: *mut ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glReadPixels(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  x,
                                                  y,
                                                  width,
                                                  height,
                                                  format,
                                                  type_,
                                                  pixels)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glReleaseShaderCompiler()```</span>
  ///
  ///
  pub fn gl_release_shader_compiler(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glReleaseShaderCompiler(self as *mut ::opengl_functions::OpenGLFunctions)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glRenderbufferStorage(unsigned int target, unsigned int internalformat, int width, int height)```</span>
  ///
  ///
  pub fn gl_renderbuffer_storage(&mut self,
                                 target: ::libc::c_uint,
                                 internalformat: ::libc::c_uint,
                                 width: ::libc::c_int,
                                 height: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glRenderbufferStorage(self as *mut ::opengl_functions::OpenGLFunctions,
                                                             target,
                                                             internalformat,
                                                             width,
                                                             height)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glSampleCoverage(float value, unsigned char invert)```</span>
  ///
  ///
  pub fn gl_sample_coverage(&mut self, value: ::libc::c_float, invert: ::libc::c_uchar) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glSampleCoverage(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        value,
                                                        invert)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glScissor(GLint x, GLint y, int width, int height)```</span>
  ///
  ///
  pub fn gl_scissor(&mut self, x: i32, y: i32, width: ::libc::c_int, height: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glScissor(self as *mut ::opengl_functions::OpenGLFunctions,
                                                 x,
                                                 y,
                                                 width,
                                                 height)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glShaderBinary(GLint n, const GLuint* shaders, unsigned int binaryformat, const void* binary, GLint length)```</span>
  ///
  ///
  pub unsafe fn gl_shader_binary(&mut self,
                                 n: i32,
                                 shaders: *const u32,
                                 binaryformat: ::libc::c_uint,
                                 binary: *const ::libc::c_void,
                                 length: i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glShaderBinary(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    n,
                                                    shaders,
                                                    binaryformat,
                                                    binary,
                                                    length)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glShaderSource(GLuint shader, int count, const char** string, const GLint* length)```</span>
  ///
  ///
  pub unsafe fn gl_shader_source(&mut self,
                                 shader: u32,
                                 count: ::libc::c_int,
                                 string: *mut *const ::libc::c_char,
                                 length: *const i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glShaderSource(self as *mut ::opengl_functions::OpenGLFunctions,
                                                    shader,
                                                    count,
                                                    string,
                                                    length)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glStencilFunc(unsigned int func, GLint ref, GLuint mask)```</span>
  ///
  ///
  pub fn gl_stencil_func(&mut self, func: ::libc::c_uint, ref_: i32, mask: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glStencilFunc(self as *mut ::opengl_functions::OpenGLFunctions,
                                                     func,
                                                     ref_,
                                                     mask)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glStencilFuncSeparate(unsigned int face, unsigned int func, GLint ref, GLuint mask)```</span>
  ///
  ///
  pub fn gl_stencil_func_separate(&mut self, face: ::libc::c_uint, func: ::libc::c_uint, ref_: i32, mask: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glStencilFuncSeparate(self as *mut ::opengl_functions::OpenGLFunctions,
                                                             face,
                                                             func,
                                                             ref_,
                                                             mask)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glStencilMask(GLuint mask)```</span>
  ///
  ///
  pub fn gl_stencil_mask(&mut self, mask: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glStencilMask(self as *mut ::opengl_functions::OpenGLFunctions, mask) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glStencilMaskSeparate(unsigned int face, GLuint mask)```</span>
  ///
  ///
  pub fn gl_stencil_mask_separate(&mut self, face: ::libc::c_uint, mask: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glStencilMaskSeparate(self as *mut ::opengl_functions::OpenGLFunctions,
                                                             face,
                                                             mask)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glStencilOp(unsigned int fail, unsigned int zfail, unsigned int zpass)```</span>
  ///
  ///
  pub fn gl_stencil_op(&mut self, fail: ::libc::c_uint, zfail: ::libc::c_uint, zpass: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glStencilOp(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   fail,
                                                   zfail,
                                                   zpass)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glStencilOpSeparate(unsigned int face, unsigned int fail, unsigned int zfail, unsigned int zpass)```</span>
  ///
  ///
  pub fn gl_stencil_op_separate(&mut self,
                                face: ::libc::c_uint,
                                fail: ::libc::c_uint,
                                zfail: ::libc::c_uint,
                                zpass: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glStencilOpSeparate(self as *mut ::opengl_functions::OpenGLFunctions,
                                                           face,
                                                           fail,
                                                           zfail,
                                                           zpass)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glTexImage2D(unsigned int target, GLint level, GLint internalformat, int width, int height, GLint border, unsigned int format, unsigned int type, const void* pixels)```</span>
  ///
  ///
  pub unsafe fn gl_tex_image_2d(&mut self,
                                target: ::libc::c_uint,
                                level: i32,
                                internalformat: i32,
                                width: ::libc::c_int,
                                height: ::libc::c_int,
                                border: i32,
                                format: ::libc::c_uint,
                                type_: ::libc::c_uint,
                                pixels: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glTexImage2D(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  target,
                                                  level,
                                                  internalformat,
                                                  width,
                                                  height,
                                                  border,
                                                  format,
                                                  type_,
                                                  pixels)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glTexParameterf(unsigned int target, unsigned int pname, float param)```</span>
  ///
  ///
  pub fn gl_tex_parameterf(&mut self, target: ::libc::c_uint, pname: ::libc::c_uint, param: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glTexParameterf(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       target,
                                                       pname,
                                                       param)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glTexParameterfv(unsigned int target, unsigned int pname, const float* params)```</span>
  ///
  ///
  pub unsafe fn gl_tex_parameterfv(&mut self,
                                   target: ::libc::c_uint,
                                   pname: ::libc::c_uint,
                                   params: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glTexParameterfv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                      target,
                                                      pname,
                                                      params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glTexParameteri(unsigned int target, unsigned int pname, GLint param)```</span>
  ///
  ///
  pub fn gl_tex_parameteri(&mut self, target: ::libc::c_uint, pname: ::libc::c_uint, param: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glTexParameteri(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       target,
                                                       pname,
                                                       param)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glTexParameteriv(unsigned int target, unsigned int pname, const GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_tex_parameteriv(&mut self, target: ::libc::c_uint, pname: ::libc::c_uint, params: *const i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glTexParameteriv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                      target,
                                                      pname,
                                                      params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glTexSubImage2D(unsigned int target, GLint level, GLint xoffset, GLint yoffset, int width, int height, unsigned int format, unsigned int type, const void* pixels)```</span>
  ///
  ///
  pub unsafe fn gl_tex_sub_image_2d(&mut self,
                                    target: ::libc::c_uint,
                                    level: i32,
                                    xoffset: i32,
                                    yoffset: i32,
                                    width: ::libc::c_int,
                                    height: ::libc::c_int,
                                    format: ::libc::c_uint,
                                    type_: ::libc::c_uint,
                                    pixels: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glTexSubImage2D(self as *mut ::opengl_functions::OpenGLFunctions,
                                                     target,
                                                     level,
                                                     xoffset,
                                                     yoffset,
                                                     width,
                                                     height,
                                                     format,
                                                     type_,
                                                     pixels)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform1f(GLint location, float x)```</span>
  ///
  ///
  pub fn gl_uniform_1f(&mut self, location: i32, x: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform1f(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform1fv(GLint location, int count, const float* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_1fv(&mut self, location: i32, count: ::libc::c_int, v: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform1fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform1i(GLint location, GLint x)```</span>
  ///
  ///
  pub fn gl_uniform_1i(&mut self, location: i32, x: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform1i(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform1iv(GLint location, int count, const GLint* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_1iv(&mut self, location: i32, count: ::libc::c_int, v: *const i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform1iv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform2f(GLint location, float x, float y)```</span>
  ///
  ///
  pub fn gl_uniform_2f(&mut self, location: i32, x: ::libc::c_float, y: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform2f(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x,
                                                   y)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform2fv(GLint location, int count, const float* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_2fv(&mut self, location: i32, count: ::libc::c_int, v: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform2fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform2i(GLint location, GLint x, GLint y)```</span>
  ///
  ///
  pub fn gl_uniform_2i(&mut self, location: i32, x: i32, y: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform2i(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x,
                                                   y)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform2iv(GLint location, int count, const GLint* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_2iv(&mut self, location: i32, count: ::libc::c_int, v: *const i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform2iv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform3f(GLint location, float x, float y, float z)```</span>
  ///
  ///
  pub fn gl_uniform_3f(&mut self, location: i32, x: ::libc::c_float, y: ::libc::c_float, z: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform3f(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x,
                                                   y,
                                                   z)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform3fv(GLint location, int count, const float* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_3fv(&mut self, location: i32, count: ::libc::c_int, v: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform3fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform3i(GLint location, GLint x, GLint y, GLint z)```</span>
  ///
  ///
  pub fn gl_uniform_3i(&mut self, location: i32, x: i32, y: i32, z: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform3i(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x,
                                                   y,
                                                   z)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform3iv(GLint location, int count, const GLint* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_3iv(&mut self, location: i32, count: ::libc::c_int, v: *const i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform3iv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform4f(GLint location, float x, float y, float z, float w)```</span>
  ///
  ///
  pub fn gl_uniform_4f(&mut self,
                       location: i32,
                       x: ::libc::c_float,
                       y: ::libc::c_float,
                       z: ::libc::c_float,
                       w: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform4f(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x,
                                                   y,
                                                   z,
                                                   w)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform4fv(GLint location, int count, const float* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_4fv(&mut self, location: i32, count: ::libc::c_int, v: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform4fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform4i(GLint location, GLint x, GLint y, GLint z, GLint w)```</span>
  ///
  ///
  pub fn gl_uniform_4i(&mut self, location: i32, x: i32, y: i32, z: i32, w: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glUniform4i(self as *mut ::opengl_functions::OpenGLFunctions,
                                                   location,
                                                   x,
                                                   y,
                                                   z,
                                                   w)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniform4iv(GLint location, int count, const GLint* v)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_4iv(&mut self, location: i32, count: ::libc::c_int, v: *const i32) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniform4iv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  location,
                                                  count,
                                                  v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniformMatrix2fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_2fv(&mut self,
                                      location: i32,
                                      count: ::libc::c_int,
                                      transpose: ::libc::c_uchar,
                                      value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniformMatrix2fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        location,
                                                        count,
                                                        transpose,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniformMatrix3fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_3fv(&mut self,
                                      location: i32,
                                      count: ::libc::c_int,
                                      transpose: ::libc::c_uchar,
                                      value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniformMatrix3fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        location,
                                                        count,
                                                        transpose,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUniformMatrix4fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_4fv(&mut self,
                                      location: i32,
                                      count: ::libc::c_int,
                                      transpose: ::libc::c_uchar,
                                      value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glUniformMatrix4fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        location,
                                                        count,
                                                        transpose,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glUseProgram(GLuint program)```</span>
  ///
  ///
  pub fn gl_use_program(&mut self, program: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_glUseProgram(self as *mut ::opengl_functions::OpenGLFunctions, program) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glValidateProgram(GLuint program)```</span>
  ///
  ///
  pub fn gl_validate_program(&mut self, program: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glValidateProgram(self as *mut ::opengl_functions::OpenGLFunctions, program)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib1f(GLuint indx, float x)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_1f(&mut self, indx: u32, x: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib1f(self as *mut ::opengl_functions::OpenGLFunctions, indx, x)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib1fv(GLuint indx, const float* values)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_1fv(&mut self, indx: u32, values: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib1fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       indx,
                                                       values)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib2f(GLuint indx, float x, float y)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_2f(&mut self, indx: u32, x: ::libc::c_float, y: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib2f(self as *mut ::opengl_functions::OpenGLFunctions, indx, x, y)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib2fv(GLuint indx, const float* values)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_2fv(&mut self, indx: u32, values: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib2fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       indx,
                                                       values)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib3f(GLuint indx, float x, float y, float z)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_3f(&mut self, indx: u32, x: ::libc::c_float, y: ::libc::c_float, z: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib3f(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        indx,
                                                        x,
                                                        y,
                                                        z)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib3fv(GLuint indx, const float* values)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_3fv(&mut self, indx: u32, values: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib3fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       indx,
                                                       values)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib4f(GLuint indx, float x, float y, float z, float w)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_4f(&mut self,
                             indx: u32,
                             x: ::libc::c_float,
                             y: ::libc::c_float,
                             z: ::libc::c_float,
                             w: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib4f(self as *mut ::opengl_functions::OpenGLFunctions,
                                                        indx,
                                                        x,
                                                        y,
                                                        z,
                                                        w)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttrib4fv(GLuint indx, const float* values)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_4fv(&mut self, indx: u32, values: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttrib4fv(self as *mut ::opengl_functions::OpenGLFunctions,
                                                       indx,
                                                       values)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glVertexAttribPointer(GLuint indx, GLint size, unsigned int type, unsigned char normalized, int stride, const void* ptr)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_pointer(&mut self,
                                         indx: u32,
                                         size: i32,
                                         type_: ::libc::c_uint,
                                         normalized: ::libc::c_uchar,
                                         stride: ::libc::c_int,
                                         ptr: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLFunctions_glVertexAttribPointer(self as *mut ::opengl_functions::OpenGLFunctions,
                                                           indx,
                                                           size,
                                                           type_,
                                                           normalized,
                                                           stride,
                                                           ptr)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::glViewport(GLint x, GLint y, int width, int height)```</span>
  ///
  ///
  pub fn gl_viewport(&mut self, x: i32, y: i32, width: ::libc::c_int, height: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_glViewport(self as *mut ::opengl_functions::OpenGLFunctions,
                                                  x,
                                                  y,
                                                  width,
                                                  height)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLFunctions::hasOpenGLFeature(QOpenGLFunctions::OpenGLFeature feature) const```</span>
  ///
  ///
  pub fn has_opengl_feature(&self, feature: &::opengl_functions::OpenGLFeature) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_hasOpenGLFeature(self as *const ::opengl_functions::OpenGLFunctions,
                                                        feature as *const ::opengl_functions::OpenGLFeature)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLFunctions::initializeOpenGLFunctions()```</span>
  ///
  ///
  pub fn initialize_opengl_functions(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLFunctions_initializeOpenGLFunctions(self as *mut ::opengl_functions::OpenGLFunctions)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFunctions::QOpenGLFunctions()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_functions::OpenGLFunctions> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLFunctions_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLFunctions::QOpenGLFunctions(QOpenGLContext* context)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(context: *mut ::opengl_context::OpenGLContext)
                           -> ::cpp_utils::CppBox<::opengl_functions::OpenGLFunctions> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLFunctions_new_context(context);
    ::cpp_utils::CppBox::new(ffi_result)
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_functions::OpenGLFunctions {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLFunctions_delete
  }
}
