/// C++ type: <span style='color: green;'>```QOpenGLExtraFunctions```</span>
#[repr(C)]
pub struct OpenGLExtraFunctions(u8);

impl OpenGLExtraFunctions {
  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glActiveShaderProgram(GLuint pipeline, GLuint program)```</span>
  ///
  ///
  pub fn gl_active_shader_program(&mut self, pipeline: u32, program: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glActiveShaderProgram(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pipeline, program) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBeginQuery(unsigned int target, GLuint id)```</span>
  ///
  ///
  pub fn gl_begin_query(&mut self, target: ::libc::c_uint, id: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBeginQuery(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                         target,
                                                         id)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBeginTransformFeedback(unsigned int primitiveMode)```</span>
  ///
  ///
  pub fn gl_begin_transform_feedback(&mut self, primitive_mode: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBeginTransformFeedback(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, primitive_mode) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindBufferBase(unsigned int target, GLuint index, GLuint buffer)```</span>
  ///
  ///
  pub fn gl_bind_buffer_base(&mut self, target: ::libc::c_uint, index: u32, buffer: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindBufferBase(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, index, buffer) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindBufferRange(unsigned int target, GLuint index, GLuint buffer, long offset, long size)```</span>
  ///
  ///
  pub fn gl_bind_buffer_range(&mut self,
                              target: ::libc::c_uint,
                              index: u32,
                              buffer: u32,
                              offset: ::libc::c_long,
                              size: ::libc::c_long) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindBufferRange(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, index, buffer, offset, size) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindImageTexture(GLuint unit, GLuint texture, GLint level, unsigned char layered, GLint layer, unsigned int access, unsigned int format)```</span>
  ///
  ///
  pub fn gl_bind_image_texture(&mut self,
                               unit: u32,
                               texture: u32,
                               level: i32,
                               layered: ::libc::c_uchar,
                               layer: i32,
                               access: ::libc::c_uint,
                               format: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindImageTexture(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, unit, texture, level, layered, layer, access, format) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindProgramPipeline(GLuint pipeline)```</span>
  ///
  ///
  pub fn gl_bind_program_pipeline(&mut self, pipeline: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindProgramPipeline(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pipeline) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindSampler(GLuint unit, GLuint sampler)```</span>
  ///
  ///
  pub fn gl_bind_sampler(&mut self, unit: u32, sampler: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindSampler(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          unit,
                                                          sampler)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindTransformFeedback(unsigned int target, GLuint id)```</span>
  ///
  ///
  pub fn gl_bind_transform_feedback(&mut self, target: ::libc::c_uint, id: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindTransformFeedback(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, id) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindVertexArray(GLuint array)```</span>
  ///
  ///
  pub fn gl_bind_vertex_array(&mut self, array: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindVertexArray(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, array) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBindVertexBuffer(GLuint bindingindex, GLuint buffer, long offset, int stride)```</span>
  ///
  ///
  pub fn gl_bind_vertex_buffer(&mut self,
                               bindingindex: u32,
                               buffer: u32,
                               offset: ::libc::c_long,
                               stride: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBindVertexBuffer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, bindingindex, buffer, offset, stride) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glBlitFramebuffer(GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, unsigned int mask, unsigned int filter)```</span>
  ///
  ///
  pub fn gl_blit_framebuffer(&mut self,
                             src_x0: i32,
                             src_y0: i32,
                             src_x1: i32,
                             src_y1: i32,
                             dst_x0: i32,
                             dst_y0: i32,
                             dst_x1: i32,
                             dst_y1: i32,
                             mask: ::libc::c_uint,
                             filter: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glBlitFramebuffer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glClearBufferfi(unsigned int buffer, GLint drawbuffer, float depth, GLint stencil)```</span>
  ///
  ///
  pub fn gl_clear_bufferfi(&mut self, buffer: ::libc::c_uint, drawbuffer: i32, depth: ::libc::c_float, stencil: i32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glClearBufferfi(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, buffer, drawbuffer, depth, stencil)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glClearBufferfv(unsigned int buffer, GLint drawbuffer, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_clear_bufferfv(&mut self, buffer: ::libc::c_uint, drawbuffer: i32, value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glClearBufferfv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          buffer,
                                                          drawbuffer,
                                                          value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glClearBufferiv(unsigned int buffer, GLint drawbuffer, const GLint* value)```</span>
  ///
  ///
  pub unsafe fn gl_clear_bufferiv(&mut self, buffer: ::libc::c_uint, drawbuffer: i32, value: *const i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glClearBufferiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          buffer,
                                                          drawbuffer,
                                                          value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glClearBufferuiv(unsigned int buffer, GLint drawbuffer, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_clear_bufferuiv(&mut self, buffer: ::libc::c_uint, drawbuffer: i32, value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glClearBufferuiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, buffer, drawbuffer, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glCompressedTexImage3D(unsigned int target, GLint level, unsigned int internalformat, int width, int height, int depth, GLint border, int imageSize, const void* data)```</span>
  ///
  ///
  pub unsafe fn gl_compressed_tex_image_3d(&mut self,
                                           target: ::libc::c_uint,
                                           level: i32,
                                           internalformat: ::libc::c_uint,
                                           width: ::libc::c_int,
                                           height: ::libc::c_int,
                                           depth: ::libc::c_int,
                                           border: i32,
                                           image_size: ::libc::c_int,
                                           data: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glCompressedTexImage3D(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, level, internalformat, width, height, depth, border, image_size, data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glCompressedTexSubImage3D(unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, int width, int height, int depth, unsigned int format, int imageSize, const void* data)```</span>
  ///
  ///
  pub unsafe fn gl_compressed_tex_sub_image_3d(&mut self,
                                               target: ::libc::c_uint,
                                               level: i32,
                                               xoffset: i32,
                                               yoffset: i32,
                                               zoffset: i32,
                                               width: ::libc::c_int,
                                               height: ::libc::c_int,
                                               depth: ::libc::c_int,
                                               format: ::libc::c_uint,
                                               image_size: ::libc::c_int,
                                               data: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glCompressedTexSubImage3D(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, level, xoffset, yoffset, zoffset, width, height, depth, format, image_size, data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glCopyBufferSubData(unsigned int readTarget, unsigned int writeTarget, long readOffset, long writeOffset, long size)```</span>
  ///
  ///
  pub fn gl_copy_buffer_sub_data(&mut self,
                                 read_target: ::libc::c_uint,
                                 write_target: ::libc::c_uint,
                                 read_offset: ::libc::c_long,
                                 write_offset: ::libc::c_long,
                                 size: ::libc::c_long) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glCopyBufferSubData(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, read_target, write_target, read_offset, write_offset, size) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glCopyTexSubImage3D(unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLint x, GLint y, int width, int height)```</span>
  ///
  ///
  pub fn gl_copy_tex_sub_image_3d(&mut self,
                                  target: ::libc::c_uint,
                                  level: i32,
                                  xoffset: i32,
                                  yoffset: i32,
                                  zoffset: i32,
                                  x: i32,
                                  y: i32,
                                  width: ::libc::c_int,
                                  height: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glCopyTexSubImage3D(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, level, xoffset, yoffset, zoffset, x, y, width, height) }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLExtraFunctions::glCreateShaderProgramv(unsigned int type, int count, const char* const * strings)```</span>
  ///
  ///
  pub unsafe fn gl_create_shader_programv(&mut self,
                                          type_: ::libc::c_uint,
                                          count: ::libc::c_int,
                                          strings: *const *const ::libc::c_char)
                                          -> u32 {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glCreateShaderProgramv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, type_, count, strings)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDeleteProgramPipelines(int n, const GLuint* pipelines)```</span>
  ///
  ///
  pub unsafe fn gl_delete_program_pipelines(&mut self, n: ::libc::c_int, pipelines: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDeleteProgramPipelines(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, n, pipelines)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDeleteQueries(int n, const GLuint* ids)```</span>
  ///
  ///
  pub unsafe fn gl_delete_queries(&mut self, n: ::libc::c_int, ids: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDeleteQueries(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          n,
                                                          ids)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDeleteSamplers(int count, const GLuint* samplers)```</span>
  ///
  ///
  pub unsafe fn gl_delete_samplers(&mut self, count: ::libc::c_int, samplers: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDeleteSamplers(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, count, samplers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDeleteTransformFeedbacks(int n, const GLuint* ids)```</span>
  ///
  ///
  pub unsafe fn gl_delete_transform_feedbacks(&mut self, n: ::libc::c_int, ids: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDeleteTransformFeedbacks(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, n, ids)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDeleteVertexArrays(int n, const GLuint* arrays)```</span>
  ///
  ///
  pub unsafe fn gl_delete_vertex_arrays(&mut self, n: ::libc::c_int, arrays: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDeleteVertexArrays(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, n, arrays)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDispatchCompute(GLuint num_groups_x, GLuint num_groups_y, GLuint num_groups_z)```</span>
  ///
  ///
  pub fn gl_dispatch_compute(&mut self, num_groups_x: u32, num_groups_y: u32, num_groups_z: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDispatchCompute(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, num_groups_x, num_groups_y, num_groups_z) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDispatchComputeIndirect(long indirect)```</span>
  ///
  ///
  pub fn gl_dispatch_compute_indirect(&mut self, indirect: ::libc::c_long) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDispatchComputeIndirect(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, indirect) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDrawArraysIndirect(unsigned int mode, const void* indirect)```</span>
  ///
  ///
  pub unsafe fn gl_draw_arrays_indirect(&mut self, mode: ::libc::c_uint, indirect: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDrawArraysIndirect(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, mode, indirect)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDrawArraysInstanced(unsigned int mode, GLint first, int count, int instancecount)```</span>
  ///
  ///
  pub fn gl_draw_arrays_instanced(&mut self,
                                  mode: ::libc::c_uint,
                                  first: i32,
                                  count: ::libc::c_int,
                                  instancecount: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDrawArraysInstanced(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, mode, first, count, instancecount) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDrawBuffers(int n, const unsigned int* bufs)```</span>
  ///
  ///
  pub unsafe fn gl_draw_buffers(&mut self, n: ::libc::c_int, bufs: *const ::libc::c_uint) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDrawBuffers(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                        n,
                                                        bufs)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDrawElementsIndirect(unsigned int mode, unsigned int type, const void* indirect)```</span>
  ///
  ///
  pub unsafe fn gl_draw_elements_indirect(&mut self,
                                          mode: ::libc::c_uint,
                                          type_: ::libc::c_uint,
                                          indirect: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDrawElementsIndirect(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, mode, type_, indirect)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDrawElementsInstanced(unsigned int mode, int count, unsigned int type, const void* indices, int instancecount)```</span>
  ///
  ///
  pub unsafe fn gl_draw_elements_instanced(&mut self,
                                           mode: ::libc::c_uint,
                                           count: ::libc::c_int,
                                           type_: ::libc::c_uint,
                                           indices: *const ::libc::c_void,
                                           instancecount: ::libc::c_int) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDrawElementsInstanced(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, mode, count, type_, indices, instancecount)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glDrawRangeElements(unsigned int mode, GLuint start, GLuint end, int count, unsigned int type, const void* indices)```</span>
  ///
  ///
  pub unsafe fn gl_draw_range_elements(&mut self,
                                       mode: ::libc::c_uint,
                                       start: u32,
                                       end: u32,
                                       count: ::libc::c_int,
                                       type_: ::libc::c_uint,
                                       indices: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glDrawRangeElements(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, mode, start, end, count, type_, indices)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glEndQuery(unsigned int target)```</span>
  ///
  ///
  pub fn gl_end_query(&mut self, target: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glEndQuery(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                       target)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glEndTransformFeedback()```</span>
  ///
  ///
  pub fn gl_end_transform_feedback(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glEndTransformFeedback(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glFlushMappedBufferRange(unsigned int target, long offset, long length)```</span>
  ///
  ///
  pub fn gl_flush_mapped_buffer_range(&mut self,
                                      target: ::libc::c_uint,
                                      offset: ::libc::c_long,
                                      length: ::libc::c_long) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glFlushMappedBufferRange(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, offset, length) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glFramebufferParameteri(unsigned int target, unsigned int pname, GLint param)```</span>
  ///
  ///
  pub fn gl_framebuffer_parameteri(&mut self, target: ::libc::c_uint, pname: ::libc::c_uint, param: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glFramebufferParameteri(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, pname, param) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glFramebufferTextureLayer(unsigned int target, unsigned int attachment, GLuint texture, GLint level, GLint layer)```</span>
  ///
  ///
  pub fn gl_framebuffer_texture_layer(&mut self,
                                      target: ::libc::c_uint,
                                      attachment: ::libc::c_uint,
                                      texture: u32,
                                      level: i32,
                                      layer: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glFramebufferTextureLayer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, attachment, texture, level, layer) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGenProgramPipelines(int n, GLuint* pipelines)```</span>
  ///
  ///
  pub unsafe fn gl_gen_program_pipelines(&mut self, n: ::libc::c_int, pipelines: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGenProgramPipelines(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, n, pipelines)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGenQueries(int n, GLuint* ids)```</span>
  ///
  ///
  pub unsafe fn gl_gen_queries(&mut self, n: ::libc::c_int, ids: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGenQueries(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                       n,
                                                       ids)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGenSamplers(int count, GLuint* samplers)```</span>
  ///
  ///
  pub unsafe fn gl_gen_samplers(&mut self, count: ::libc::c_int, samplers: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGenSamplers(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                        count,
                                                        samplers)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGenTransformFeedbacks(int n, GLuint* ids)```</span>
  ///
  ///
  pub unsafe fn gl_gen_transform_feedbacks(&mut self, n: ::libc::c_int, ids: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGenTransformFeedbacks(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, n, ids)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGenVertexArrays(int n, GLuint* arrays)```</span>
  ///
  ///
  pub unsafe fn gl_gen_vertex_arrays(&mut self, n: ::libc::c_int, arrays: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGenVertexArrays(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, n, arrays)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetActiveUniformBlockName(GLuint program, GLuint uniformBlockIndex, int bufSize, int* length, char* uniformBlockName)```</span>
  ///
  ///
  pub unsafe fn gl_get_active_uniform_block_name(&mut self,
                                                 program: u32,
                                                 uniform_block_index: u32,
                                                 buf_size: ::libc::c_int,
                                                 length: *mut ::libc::c_int,
                                                 uniform_block_name: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformBlockName(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, uniform_block_index, buf_size, length, uniform_block_name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetActiveUniformBlockiv(GLuint program, GLuint uniformBlockIndex, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_active_uniform_blockiv(&mut self,
                                              program: u32,
                                              uniform_block_index: u32,
                                              pname: ::libc::c_uint,
                                              params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformBlockiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, uniform_block_index, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetActiveUniformsiv(GLuint program, int uniformCount, const GLuint* uniformIndices, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_active_uniformsiv(&mut self,
                                         program: u32,
                                         uniform_count: ::libc::c_int,
                                         uniform_indices: *const u32,
                                         pname: ::libc::c_uint,
                                         params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformsiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, uniform_count, uniform_indices, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetBooleani_v(unsigned int target, GLuint index, unsigned char* data)```</span>
  ///
  ///
  pub unsafe fn gl_get_booleani_v(&mut self, target: ::libc::c_uint, index: u32, data: *mut ::libc::c_uchar) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetBooleani_v(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          target,
                                                          index,
                                                          data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetBufferParameteri64v(unsigned int target, unsigned int pname, GLint64* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_buffer_parameteri_64v(&mut self,
                                             target: ::libc::c_uint,
                                             pname: ::libc::c_uint,
                                             params: *mut i64) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetBufferParameteri64v(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetBufferPointerv(unsigned int target, unsigned int pname, void** params)```</span>
  ///
  ///
  pub unsafe fn gl_get_buffer_pointerv(&mut self,
                                       target: ::libc::c_uint,
                                       pname: ::libc::c_uint,
                                       params: *mut *mut ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetBufferPointerv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, pname, params)
  }

  /// C++ method: <span style='color: green;'>```GLint QOpenGLExtraFunctions::glGetFragDataLocation(GLuint program, const char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_frag_data_location(&mut self, program: u32, name: *const ::libc::c_char) -> i32 {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetFragDataLocation(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetFramebufferParameteriv(unsigned int target, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_framebuffer_parameteriv(&mut self,
                                               target: ::libc::c_uint,
                                               pname: ::libc::c_uint,
                                               params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetFramebufferParameteriv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetInteger64i_v(unsigned int target, GLuint index, GLint64* data)```</span>
  ///
  ///
  pub unsafe fn gl_get_integer_64i_v(&mut self, target: ::libc::c_uint, index: u32, data: *mut i64) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetInteger64i_v(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, index, data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetInteger64v(unsigned int pname, GLint64* data)```</span>
  ///
  ///
  pub unsafe fn gl_get_integer_64v(&mut self, pname: ::libc::c_uint, data: *mut i64) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetInteger64v(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          pname,
                                                          data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetIntegeri_v(unsigned int target, GLuint index, GLint* data)```</span>
  ///
  ///
  pub unsafe fn gl_get_integeri_v(&mut self, target: ::libc::c_uint, index: u32, data: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetIntegeri_v(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          target,
                                                          index,
                                                          data)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetInternalformativ(unsigned int target, unsigned int internalformat, unsigned int pname, int bufSize, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_internalformativ(&mut self,
                                        target: ::libc::c_uint,
                                        internalformat: ::libc::c_uint,
                                        pname: ::libc::c_uint,
                                        buf_size: ::libc::c_int,
                                        params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetInternalformativ(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, internalformat, pname, buf_size, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetMultisamplefv(unsigned int pname, GLuint index, float* val)```</span>
  ///
  ///
  pub unsafe fn gl_get_multisamplefv(&mut self, pname: ::libc::c_uint, index: u32, val: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetMultisamplefv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pname, index, val)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetProgramBinary(GLuint program, int bufSize, int* length, unsigned int* binaryFormat, void* binary)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_binary(&mut self,
                                      program: u32,
                                      buf_size: ::libc::c_int,
                                      length: *mut ::libc::c_int,
                                      binary_format: *mut ::libc::c_uint,
                                      binary: *mut ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramBinary(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, buf_size, length, binary_format, binary)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetProgramInterfaceiv(GLuint program, unsigned int programInterface, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_interfaceiv(&mut self,
                                           program: u32,
                                           program_interface: ::libc::c_uint,
                                           pname: ::libc::c_uint,
                                           params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramInterfaceiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, program_interface, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetProgramPipelineInfoLog(GLuint pipeline, int bufSize, int* length, char* infoLog)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_pipeline_info_log(&mut self,
                                                 pipeline: u32,
                                                 buf_size: ::libc::c_int,
                                                 length: *mut ::libc::c_int,
                                                 info_log: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramPipelineInfoLog(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pipeline, buf_size, length, info_log)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetProgramPipelineiv(GLuint pipeline, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_pipelineiv(&mut self, pipeline: u32, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramPipelineiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pipeline, pname, params)
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLExtraFunctions::glGetProgramResourceIndex(GLuint program, unsigned int programInterface, const char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_resource_index(&mut self,
                                              program: u32,
                                              program_interface: ::libc::c_uint,
                                              name: *const ::libc::c_char)
                                              -> u32 {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceIndex(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, program_interface, name)
  }

  /// C++ method: <span style='color: green;'>```GLint QOpenGLExtraFunctions::glGetProgramResourceLocation(GLuint program, unsigned int programInterface, const char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_resource_location(&mut self,
                                                 program: u32,
                                                 program_interface: ::libc::c_uint,
                                                 name: *const ::libc::c_char)
                                                 -> i32 {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceLocation(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, program_interface, name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetProgramResourceName(GLuint program, unsigned int programInterface, GLuint index, int bufSize, int* length, char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_resource_name(&mut self,
                                             program: u32,
                                             program_interface: ::libc::c_uint,
                                             index: u32,
                                             buf_size: ::libc::c_int,
                                             length: *mut ::libc::c_int,
                                             name: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceName(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, program_interface, index, buf_size, length, name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetProgramResourceiv(GLuint program, unsigned int programInterface, GLuint index, int propCount, const unsigned int* props, int bufSize, int* length, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_program_resourceiv(&mut self,
                                          program: u32,
                                          program_interface: ::libc::c_uint,
                                          index: u32,
                                          prop_count: ::libc::c_int,
                                          props: *const ::libc::c_uint,
                                          buf_size: ::libc::c_int,
                                          length: *mut ::libc::c_int,
                                          params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, program_interface, index, prop_count, props, buf_size, length, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetQueryObjectuiv(GLuint id, unsigned int pname, GLuint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_query_objectuiv(&mut self, id: u32, pname: ::libc::c_uint, params: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetQueryObjectuiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, id, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetQueryiv(unsigned int target, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_queryiv(&mut self, target: ::libc::c_uint, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetQueryiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                       target,
                                                       pname,
                                                       params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetSamplerParameterfv(GLuint sampler, unsigned int pname, float* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_sampler_parameterfv(&mut self,
                                           sampler: u32,
                                           pname: ::libc::c_uint,
                                           params: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetSamplerParameterfv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, sampler, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetSamplerParameteriv(GLuint sampler, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_sampler_parameteriv(&mut self, sampler: u32, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetSamplerParameteriv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, sampler, pname, params)
  }

  /// C++ method: <span style='color: green;'>```const GLubyte* QOpenGLExtraFunctions::glGetStringi(unsigned int name, GLuint index)```</span>
  ///
  ///
  pub fn gl_get_stringi(&mut self, name: ::libc::c_uint, index: u32) -> *const u8 {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetStringi(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                         name,
                                                         index)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetTexLevelParameterfv(unsigned int target, GLint level, unsigned int pname, float* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_tex_level_parameterfv(&mut self,
                                             target: ::libc::c_uint,
                                             level: i32,
                                             pname: ::libc::c_uint,
                                             params: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetTexLevelParameterfv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, level, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetTexLevelParameteriv(unsigned int target, GLint level, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_tex_level_parameteriv(&mut self,
                                             target: ::libc::c_uint,
                                             level: i32,
                                             pname: ::libc::c_uint,
                                             params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetTexLevelParameteriv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, level, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetTransformFeedbackVarying(GLuint program, GLuint index, int bufSize, int* length, int* size, unsigned int* type, char* name)```</span>
  ///
  ///
  pub unsafe fn gl_get_transform_feedback_varying(&mut self,
                                                  program: u32,
                                                  index: u32,
                                                  buf_size: ::libc::c_int,
                                                  length: *mut ::libc::c_int,
                                                  size: *mut ::libc::c_int,
                                                  type_: *mut ::libc::c_uint,
                                                  name: *mut ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetTransformFeedbackVarying(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, index, buf_size, length, size, type_, name)
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLExtraFunctions::glGetUniformBlockIndex(GLuint program, const char* uniformBlockName)```</span>
  ///
  ///
  pub unsafe fn gl_get_uniform_block_index(&mut self, program: u32, uniform_block_name: *const ::libc::c_char) -> u32 {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetUniformBlockIndex(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, uniform_block_name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetUniformIndices(GLuint program, int uniformCount, const char* const * uniformNames, GLuint* uniformIndices)```</span>
  ///
  ///
  pub unsafe fn gl_get_uniform_indices(&mut self,
                                       program: u32,
                                       uniform_count: ::libc::c_int,
                                       uniform_names: *const *const ::libc::c_char,
                                       uniform_indices: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetUniformIndices(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, uniform_count, uniform_names, uniform_indices)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetUniformuiv(GLuint program, GLint location, GLuint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_uniformuiv(&mut self, program: u32, location: i32, params: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetUniformuiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          program,
                                                          location,
                                                          params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetVertexAttribIiv(GLuint index, unsigned int pname, GLint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_vertex_attrib_iiv(&mut self, index: u32, pname: ::libc::c_uint, params: *mut i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetVertexAttribIiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glGetVertexAttribIuiv(GLuint index, unsigned int pname, GLuint* params)```</span>
  ///
  ///
  pub unsafe fn gl_get_vertex_attrib_iuiv(&mut self, index: u32, pname: ::libc::c_uint, params: *mut u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glGetVertexAttribIuiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, pname, params)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glInvalidateFramebuffer(unsigned int target, int numAttachments, const unsigned int* attachments)```</span>
  ///
  ///
  pub unsafe fn gl_invalidate_framebuffer(&mut self,
                                          target: ::libc::c_uint,
                                          num_attachments: ::libc::c_int,
                                          attachments: *const ::libc::c_uint) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glInvalidateFramebuffer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, num_attachments, attachments)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glInvalidateSubFramebuffer(unsigned int target, int numAttachments, const unsigned int* attachments, GLint x, GLint y, int width, int height)```</span>
  ///
  ///
  pub unsafe fn gl_invalidate_sub_framebuffer(&mut self,
                                              target: ::libc::c_uint,
                                              num_attachments: ::libc::c_int,
                                              attachments: *const ::libc::c_uint,
                                              x: i32,
                                              y: i32,
                                              width: ::libc::c_int,
                                              height: ::libc::c_int) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glInvalidateSubFramebuffer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, num_attachments, attachments, x, y, width, height)
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLExtraFunctions::glIsProgramPipeline(GLuint pipeline)```</span>
  ///
  ///
  pub fn gl_is_program_pipeline(&mut self, pipeline: u32) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glIsProgramPipeline(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pipeline) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLExtraFunctions::glIsQuery(GLuint id)```</span>
  ///
  ///
  pub fn gl_is_query(&mut self, id: u32) -> ::libc::c_uchar {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glIsQuery(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                      id)
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLExtraFunctions::glIsSampler(GLuint sampler)```</span>
  ///
  ///
  pub fn gl_is_sampler(&mut self, sampler: u32) -> ::libc::c_uchar {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glIsSampler(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                        sampler)
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLExtraFunctions::glIsTransformFeedback(GLuint id)```</span>
  ///
  ///
  pub fn gl_is_transform_feedback(&mut self, id: u32) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glIsTransformFeedback(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, id) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLExtraFunctions::glIsVertexArray(GLuint array)```</span>
  ///
  ///
  pub fn gl_is_vertex_array(&mut self, array: u32) -> ::libc::c_uchar {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glIsVertexArray(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, array)
    }
  }

  /// C++ method: <span style='color: green;'>```void* QOpenGLExtraFunctions::glMapBufferRange(unsigned int target, long offset, long length, unsigned int access)```</span>
  ///
  ///
  pub fn gl_map_buffer_range(&mut self,
                             target: ::libc::c_uint,
                             offset: ::libc::c_long,
                             length: ::libc::c_long,
                             access: ::libc::c_uint)
                             -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glMapBufferRange(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, offset, length, access) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glMemoryBarrier(unsigned int barriers)```</span>
  ///
  ///
  pub fn gl_memory_barrier(&mut self, barriers: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glMemoryBarrier(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, barriers)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glMemoryBarrierByRegion(unsigned int barriers)```</span>
  ///
  ///
  pub fn gl_memory_barrier_by_region(&mut self, barriers: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glMemoryBarrierByRegion(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, barriers) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glPauseTransformFeedback()```</span>
  ///
  ///
  pub fn gl_pause_transform_feedback(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glPauseTransformFeedback(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramBinary(GLuint program, unsigned int binaryFormat, const void* binary, int length)```</span>
  ///
  ///
  pub unsafe fn gl_program_binary(&mut self,
                                  program: u32,
                                  binary_format: ::libc::c_uint,
                                  binary: *const ::libc::c_void,
                                  length: ::libc::c_int) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramBinary(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          program,
                                                          binary_format,
                                                          binary,
                                                          length)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramParameteri(GLuint program, unsigned int pname, GLint value)```</span>
  ///
  ///
  pub fn gl_program_parameteri(&mut self, program: u32, pname: ::libc::c_uint, value: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramParameteri(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, pname, value) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform1f(GLuint program, GLint location, float v0)```</span>
  ///
  ///
  pub fn gl_program_uniform_1f(&mut self, program: u32, location: i32, v0: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1f(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform1fv(GLuint program, GLint location, int count, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_1fv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform1i(GLuint program, GLint location, GLint v0)```</span>
  ///
  ///
  pub fn gl_program_uniform_1i(&mut self, program: u32, location: i32, v0: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1i(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform1iv(GLuint program, GLint location, int count, const GLint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_1iv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1iv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform1ui(GLuint program, GLint location, GLuint v0)```</span>
  ///
  ///
  pub fn gl_program_uniform_1ui(&mut self, program: u32, location: i32, v0: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform1uiv(GLuint program, GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_1uiv(&mut self,
                                        program: u32,
                                        location: i32,
                                        count: ::libc::c_int,
                                        value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform2f(GLuint program, GLint location, float v0, float v1)```</span>
  ///
  ///
  pub fn gl_program_uniform_2f(&mut self, program: u32, location: i32, v0: ::libc::c_float, v1: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2f(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform2fv(GLuint program, GLint location, int count, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_2fv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform2i(GLuint program, GLint location, GLint v0, GLint v1)```</span>
  ///
  ///
  pub fn gl_program_uniform_2i(&mut self, program: u32, location: i32, v0: i32, v1: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2i(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform2iv(GLuint program, GLint location, int count, const GLint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_2iv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2iv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform2ui(GLuint program, GLint location, GLuint v0, GLuint v1)```</span>
  ///
  ///
  pub fn gl_program_uniform_2ui(&mut self, program: u32, location: i32, v0: u32, v1: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform2uiv(GLuint program, GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_2uiv(&mut self,
                                        program: u32,
                                        location: i32,
                                        count: ::libc::c_int,
                                        value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform3f(GLuint program, GLint location, float v0, float v1, float v2)```</span>
  ///
  ///
  pub fn gl_program_uniform_3f(&mut self,
                               program: u32,
                               location: i32,
                               v0: ::libc::c_float,
                               v1: ::libc::c_float,
                               v2: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3f(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1, v2) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform3fv(GLuint program, GLint location, int count, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_3fv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform3i(GLuint program, GLint location, GLint v0, GLint v1, GLint v2)```</span>
  ///
  ///
  pub fn gl_program_uniform_3i(&mut self, program: u32, location: i32, v0: i32, v1: i32, v2: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3i(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1, v2) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform3iv(GLuint program, GLint location, int count, const GLint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_3iv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3iv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform3ui(GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2)```</span>
  ///
  ///
  pub fn gl_program_uniform_3ui(&mut self, program: u32, location: i32, v0: u32, v1: u32, v2: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1, v2) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform3uiv(GLuint program, GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_3uiv(&mut self,
                                        program: u32,
                                        location: i32,
                                        count: ::libc::c_int,
                                        value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform4f(GLuint program, GLint location, float v0, float v1, float v2, float v3)```</span>
  ///
  ///
  pub fn gl_program_uniform_4f(&mut self,
                               program: u32,
                               location: i32,
                               v0: ::libc::c_float,
                               v1: ::libc::c_float,
                               v2: ::libc::c_float,
                               v3: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4f(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1, v2, v3) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform4fv(GLuint program, GLint location, int count, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_4fv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform4i(GLuint program, GLint location, GLint v0, GLint v1, GLint v2, GLint v3)```</span>
  ///
  ///
  pub fn gl_program_uniform_4i(&mut self, program: u32, location: i32, v0: i32, v1: i32, v2: i32, v3: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4i(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1, v2, v3) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform4iv(GLuint program, GLint location, int count, const GLint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_4iv(&mut self,
                                       program: u32,
                                       location: i32,
                                       count: ::libc::c_int,
                                       value: *const i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4iv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform4ui(GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3)```</span>
  ///
  ///
  pub fn gl_program_uniform_4ui(&mut self, program: u32, location: i32, v0: u32, v1: u32, v2: u32, v3: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, v0, v1, v2, v3) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniform4uiv(GLuint program, GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_4uiv(&mut self,
                                        program: u32,
                                        location: i32,
                                        count: ::libc::c_int,
                                        value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix2fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_2fv(&mut self,
                                              program: u32,
                                              location: i32,
                                              count: ::libc::c_int,
                                              transpose: ::libc::c_uchar,
                                              value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix2x3fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_2x_3fv(&mut self,
                                                 program: u32,
                                                 location: i32,
                                                 count: ::libc::c_int,
                                                 transpose: ::libc::c_uchar,
                                                 value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2x3fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix2x4fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_2x_4fv(&mut self,
                                                 program: u32,
                                                 location: i32,
                                                 count: ::libc::c_int,
                                                 transpose: ::libc::c_uchar,
                                                 value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2x4fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix3fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_3fv(&mut self,
                                              program: u32,
                                              location: i32,
                                              count: ::libc::c_int,
                                              transpose: ::libc::c_uchar,
                                              value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix3x2fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_3x_2fv(&mut self,
                                                 program: u32,
                                                 location: i32,
                                                 count: ::libc::c_int,
                                                 transpose: ::libc::c_uchar,
                                                 value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3x2fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix3x4fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_3x_4fv(&mut self,
                                                 program: u32,
                                                 location: i32,
                                                 count: ::libc::c_int,
                                                 transpose: ::libc::c_uchar,
                                                 value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3x4fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix4fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_4fv(&mut self,
                                              program: u32,
                                              location: i32,
                                              count: ::libc::c_int,
                                              transpose: ::libc::c_uchar,
                                              value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix4x2fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_4x_2fv(&mut self,
                                                 program: u32,
                                                 location: i32,
                                                 count: ::libc::c_int,
                                                 transpose: ::libc::c_uchar,
                                                 value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4x2fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glProgramUniformMatrix4x3fv(GLuint program, GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_program_uniform_matrix_4x_3fv(&mut self,
                                                 program: u32,
                                                 location: i32,
                                                 count: ::libc::c_int,
                                                 transpose: ::libc::c_uchar,
                                                 value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4x3fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glReadBuffer(unsigned int mode)```</span>
  ///
  ///
  pub fn gl_read_buffer(&mut self, mode: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glReadBuffer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                         mode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glRenderbufferStorageMultisample(unsigned int target, int samples, unsigned int internalformat, int width, int height)```</span>
  ///
  ///
  pub fn gl_renderbuffer_storage_multisample(&mut self,
                                             target: ::libc::c_uint,
                                             samples: ::libc::c_int,
                                             internalformat: ::libc::c_uint,
                                             width: ::libc::c_int,
                                             height: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glRenderbufferStorageMultisample(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, samples, internalformat, width, height) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glResumeTransformFeedback()```</span>
  ///
  ///
  pub fn gl_resume_transform_feedback(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glResumeTransformFeedback(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glSampleMaski(GLuint maskNumber, unsigned int mask)```</span>
  ///
  ///
  pub fn gl_sample_maski(&mut self, mask_number: u32, mask: ::libc::c_uint) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glSampleMaski(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          mask_number,
                                                          mask)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glSamplerParameterf(GLuint sampler, unsigned int pname, float param)```</span>
  ///
  ///
  pub fn gl_sampler_parameterf(&mut self, sampler: u32, pname: ::libc::c_uint, param: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glSamplerParameterf(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, sampler, pname, param) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glSamplerParameterfv(GLuint sampler, unsigned int pname, const float* param)```</span>
  ///
  ///
  pub unsafe fn gl_sampler_parameterfv(&mut self, sampler: u32, pname: ::libc::c_uint, param: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glSamplerParameterfv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, sampler, pname, param)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glSamplerParameteri(GLuint sampler, unsigned int pname, GLint param)```</span>
  ///
  ///
  pub fn gl_sampler_parameteri(&mut self, sampler: u32, pname: ::libc::c_uint, param: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glSamplerParameteri(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, sampler, pname, param) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glSamplerParameteriv(GLuint sampler, unsigned int pname, const GLint* param)```</span>
  ///
  ///
  pub unsafe fn gl_sampler_parameteriv(&mut self, sampler: u32, pname: ::libc::c_uint, param: *const i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glSamplerParameteriv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, sampler, pname, param)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glTexImage3D(unsigned int target, GLint level, GLint internalformat, int width, int height, int depth, GLint border, unsigned int format, unsigned int type, const void* pixels)```</span>
  ///
  ///
  pub unsafe fn gl_tex_image_3d(&mut self,
                                target: ::libc::c_uint,
                                level: i32,
                                internalformat: i32,
                                width: ::libc::c_int,
                                height: ::libc::c_int,
                                depth: ::libc::c_int,
                                border: i32,
                                format: ::libc::c_uint,
                                type_: ::libc::c_uint,
                                pixels: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glTexImage3D(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                       target,
                                                       level,
                                                       internalformat,
                                                       width,
                                                       height,
                                                       depth,
                                                       border,
                                                       format,
                                                       type_,
                                                       pixels)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glTexStorage2D(unsigned int target, int levels, unsigned int internalformat, int width, int height)```</span>
  ///
  ///
  pub fn gl_tex_storage_2d(&mut self,
                           target: ::libc::c_uint,
                           levels: ::libc::c_int,
                           internalformat: ::libc::c_uint,
                           width: ::libc::c_int,
                           height: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glTexStorage2D(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, levels, internalformat, width, height)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glTexStorage2DMultisample(unsigned int target, int samples, unsigned int internalformat, int width, int height, unsigned char fixedsamplelocations)```</span>
  ///
  ///
  pub fn gl_tex_storage_2d_multisample(&mut self,
                                       target: ::libc::c_uint,
                                       samples: ::libc::c_int,
                                       internalformat: ::libc::c_uint,
                                       width: ::libc::c_int,
                                       height: ::libc::c_int,
                                       fixedsamplelocations: ::libc::c_uchar) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glTexStorage2DMultisample(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, samples, internalformat, width, height, fixedsamplelocations) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glTexStorage3D(unsigned int target, int levels, unsigned int internalformat, int width, int height, int depth)```</span>
  ///
  ///
  pub fn gl_tex_storage_3d(&mut self,
                           target: ::libc::c_uint,
                           levels: ::libc::c_int,
                           internalformat: ::libc::c_uint,
                           width: ::libc::c_int,
                           height: ::libc::c_int,
                           depth: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glTexStorage3D(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, target, levels, internalformat, width, height, depth)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glTexSubImage3D(unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, int width, int height, int depth, unsigned int format, unsigned int type, const void* pixels)```</span>
  ///
  ///
  pub unsafe fn gl_tex_sub_image_3d(&mut self,
                                    target: ::libc::c_uint,
                                    level: i32,
                                    xoffset: i32,
                                    yoffset: i32,
                                    zoffset: i32,
                                    width: ::libc::c_int,
                                    height: ::libc::c_int,
                                    depth: ::libc::c_int,
                                    format: ::libc::c_uint,
                                    type_: ::libc::c_uint,
                                    pixels: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glTexSubImage3D(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          target,
                                                          level,
                                                          xoffset,
                                                          yoffset,
                                                          zoffset,
                                                          width,
                                                          height,
                                                          depth,
                                                          format,
                                                          type_,
                                                          pixels)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glTransformFeedbackVaryings(GLuint program, int count, const char* const * varyings, unsigned int bufferMode)```</span>
  ///
  ///
  pub unsafe fn gl_transform_feedback_varyings(&mut self,
                                               program: u32,
                                               count: ::libc::c_int,
                                               varyings: *const *const ::libc::c_char,
                                               buffer_mode: ::libc::c_uint) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glTransformFeedbackVaryings(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, count, varyings, buffer_mode)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform1ui(GLint location, GLuint v0)```</span>
  ///
  ///
  pub fn gl_uniform_1ui(&mut self, location: i32, v0: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform1ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                         location,
                                                         v0)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform1uiv(GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_1uiv(&mut self, location: i32, count: ::libc::c_int, value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform1uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                        location,
                                                        count,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform2ui(GLint location, GLuint v0, GLuint v1)```</span>
  ///
  ///
  pub fn gl_uniform_2ui(&mut self, location: i32, v0: u32, v1: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform2ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                         location,
                                                         v0,
                                                         v1)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform2uiv(GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_2uiv(&mut self, location: i32, count: ::libc::c_int, value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform2uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                        location,
                                                        count,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform3ui(GLint location, GLuint v0, GLuint v1, GLuint v2)```</span>
  ///
  ///
  pub fn gl_uniform_3ui(&mut self, location: i32, v0: u32, v1: u32, v2: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform3ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                         location,
                                                         v0,
                                                         v1,
                                                         v2)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform3uiv(GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_3uiv(&mut self, location: i32, count: ::libc::c_int, value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform3uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                        location,
                                                        count,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform4ui(GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3)```</span>
  ///
  ///
  pub fn gl_uniform_4ui(&mut self, location: i32, v0: u32, v1: u32, v2: u32, v3: u32) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform4ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                         location,
                                                         v0,
                                                         v1,
                                                         v2,
                                                         v3)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniform4uiv(GLint location, int count, const GLuint* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_4uiv(&mut self, location: i32, count: ::libc::c_int, value: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniform4uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                        location,
                                                        count,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniformBlockBinding(GLuint program, GLuint uniformBlockIndex, GLuint uniformBlockBinding)```</span>
  ///
  ///
  pub fn gl_uniform_block_binding(&mut self, program: u32, uniform_block_index: u32, uniform_block_binding: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniformBlockBinding(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, program, uniform_block_index, uniform_block_binding) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniformMatrix2x3fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_2x_3fv(&mut self,
                                         location: i32,
                                         count: ::libc::c_int,
                                         transpose: ::libc::c_uchar,
                                         value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix2x3fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniformMatrix2x4fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_2x_4fv(&mut self,
                                         location: i32,
                                         count: ::libc::c_int,
                                         transpose: ::libc::c_uchar,
                                         value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix2x4fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniformMatrix3x2fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_3x_2fv(&mut self,
                                         location: i32,
                                         count: ::libc::c_int,
                                         transpose: ::libc::c_uchar,
                                         value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix3x2fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniformMatrix3x4fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_3x_4fv(&mut self,
                                         location: i32,
                                         count: ::libc::c_int,
                                         transpose: ::libc::c_uchar,
                                         value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix3x4fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniformMatrix4x2fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_4x_2fv(&mut self,
                                         location: i32,
                                         count: ::libc::c_int,
                                         transpose: ::libc::c_uchar,
                                         value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix4x2fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUniformMatrix4x3fv(GLint location, int count, unsigned char transpose, const float* value)```</span>
  ///
  ///
  pub unsafe fn gl_uniform_matrix_4x_3fv(&mut self,
                                         location: i32,
                                         count: ::libc::c_int,
                                         transpose: ::libc::c_uchar,
                                         value: *const ::libc::c_float) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix4x3fv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, location, count, transpose, value)
  }

  /// C++ method: <span style='color: green;'>```unsigned char QOpenGLExtraFunctions::glUnmapBuffer(unsigned int target)```</span>
  ///
  ///
  pub fn gl_unmap_buffer(&mut self, target: ::libc::c_uint) -> ::libc::c_uchar {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUnmapBuffer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions,
                                                          target)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glUseProgramStages(GLuint pipeline, unsigned int stages, GLuint program)```</span>
  ///
  ///
  pub fn gl_use_program_stages(&mut self, pipeline: u32, stages: ::libc::c_uint, program: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glUseProgramStages(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pipeline, stages, program) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glValidateProgramPipeline(GLuint pipeline)```</span>
  ///
  ///
  pub fn gl_validate_program_pipeline(&mut self, pipeline: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glValidateProgramPipeline(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, pipeline) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribBinding(GLuint attribindex, GLuint bindingindex)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_binding(&mut self, attribindex: u32, bindingindex: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribBinding(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, attribindex, bindingindex) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribDivisor(GLuint index, GLuint divisor)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_divisor(&mut self, index: u32, divisor: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribDivisor(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, divisor) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribFormat(GLuint attribindex, GLint size, unsigned int type, unsigned char normalized, GLuint relativeoffset)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_format(&mut self,
                                 attribindex: u32,
                                 size: i32,
                                 type_: ::libc::c_uint,
                                 normalized: ::libc::c_uchar,
                                 relativeoffset: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribFormat(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, attribindex, size, type_, normalized, relativeoffset) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribI4i(GLuint index, GLint x, GLint y, GLint z, GLint w)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_i4i(&mut self, index: u32, x: i32, y: i32, z: i32, w: i32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4i(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, x, y, z, w) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribI4iv(GLuint index, const GLint* v)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_i4iv(&mut self, index: u32, v: *const i32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4iv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribI4ui(GLuint index, GLuint x, GLuint y, GLuint z, GLuint w)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_i4ui(&mut self, index: u32, x: u32, y: u32, z: u32, w: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4ui(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, x, y, z, w) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribI4uiv(GLuint index, const GLuint* v)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_i4uiv(&mut self, index: u32, v: *const u32) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4uiv(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, v)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribIFormat(GLuint attribindex, GLint size, unsigned int type, GLuint relativeoffset)```</span>
  ///
  ///
  pub fn gl_vertex_attrib_i_format(&mut self,
                                   attribindex: u32,
                                   size: i32,
                                   type_: ::libc::c_uint,
                                   relativeoffset: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribIFormat(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, attribindex, size, type_, relativeoffset) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexAttribIPointer(GLuint index, GLint size, unsigned int type, int stride, const void* pointer)```</span>
  ///
  ///
  pub unsafe fn gl_vertex_attrib_i_pointer(&mut self,
                                           index: u32,
                                           size: i32,
                                           type_: ::libc::c_uint,
                                           stride: ::libc::c_int,
                                           pointer: *const ::libc::c_void) {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexAttribIPointer(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, index, size, type_, stride, pointer)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLExtraFunctions::glVertexBindingDivisor(GLuint bindingindex, GLuint divisor)```</span>
  ///
  ///
  pub fn gl_vertex_binding_divisor(&mut self, bindingindex: u32, divisor: u32) {
    unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_glVertexBindingDivisor(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions, bindingindex, divisor) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLExtraFunctions::QOpenGLExtraFunctions()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_extra_functions::OpenGLExtraFunctions> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLExtraFunctions::QOpenGLExtraFunctions(QOpenGLContext* context)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(context: *mut ::opengl_context::OpenGLContext)
                           -> ::cpp_utils::CppBox<::opengl_extra_functions::OpenGLExtraFunctions> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLExtraFunctions_new_context(context);
    ::cpp_utils::CppBox::new(ffi_result)
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_extra_functions::OpenGLExtraFunctions {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLExtraFunctions_delete
  }
}

impl ::cpp_utils::StaticCast<::opengl_functions::OpenGLFunctions> for ::opengl_extra_functions::OpenGLExtraFunctions {
  fn static_cast_mut(&mut self) -> &mut ::opengl_functions::OpenGLFunctions {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLFunctions_ptr(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::opengl_functions::OpenGLFunctions {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLFunctions_ptr(self as *const ::opengl_extra_functions::OpenGLExtraFunctions as *mut ::opengl_extra_functions::OpenGLExtraFunctions) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_extra_functions::OpenGLExtraFunctions> for ::opengl_functions::OpenGLFunctions {
unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_extra_functions::OpenGLExtraFunctions {
let ffi_result = ::ffi::qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLExtraFunctions_ptr(self as *mut ::opengl_functions::OpenGLFunctions);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::opengl_extra_functions::OpenGLExtraFunctions {
let ffi_result = ::ffi::qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLExtraFunctions_ptr(self as *const ::opengl_functions::OpenGLFunctions as *mut ::opengl_functions::OpenGLFunctions);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::opengl_extra_functions::OpenGLExtraFunctions {
  type Target = ::opengl_functions::OpenGLFunctions;
  fn deref(&self) -> &::opengl_functions::OpenGLFunctions {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLFunctions_ptr(self as *const ::opengl_extra_functions::OpenGLExtraFunctions as *mut ::opengl_extra_functions::OpenGLExtraFunctions) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_extra_functions::OpenGLExtraFunctions {
  fn deref_mut(&mut self) -> &mut ::opengl_functions::OpenGLFunctions {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLFunctions_ptr(self as *mut ::opengl_extra_functions::OpenGLExtraFunctions) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
