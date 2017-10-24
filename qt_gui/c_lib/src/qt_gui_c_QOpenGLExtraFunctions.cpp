#include "qt_gui_c_QOpenGLExtraFunctions.h"

QOpenGLExtraFunctions* qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLExtraFunctions_ptr(QOpenGLFunctions* ptr) {
  return static_cast<QOpenGLExtraFunctions*>(ptr);
}

QOpenGLFunctions* qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLFunctions_ptr(QOpenGLExtraFunctions* ptr) {
  return static_cast<QOpenGLFunctions*>(ptr);
}

void qt_gui_c_QOpenGLExtraFunctions_delete(QOpenGLExtraFunctions* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLExtraFunctions_glActiveShaderProgram(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, GLuint program) {
  this_ptr->glActiveShaderProgram(pipeline, program);
}

void qt_gui_c_QOpenGLExtraFunctions_glBeginQuery(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint id) {
  this_ptr->glBeginQuery(target, id);
}

void qt_gui_c_QOpenGLExtraFunctions_glBeginTransformFeedback(QOpenGLExtraFunctions* this_ptr, unsigned int primitiveMode) {
  this_ptr->glBeginTransformFeedback(primitiveMode);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindBufferBase(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLuint buffer) {
  this_ptr->glBindBufferBase(target, index, buffer);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindBufferRange(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLuint buffer, long offset, long size) {
  this_ptr->glBindBufferRange(target, index, buffer, offset, size);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindImageTexture(QOpenGLExtraFunctions* this_ptr, GLuint unit, GLuint texture, GLint level, unsigned char layered, GLint layer, unsigned int access, unsigned int format) {
  this_ptr->glBindImageTexture(unit, texture, level, layered, layer, access, format);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindProgramPipeline(QOpenGLExtraFunctions* this_ptr, GLuint pipeline) {
  this_ptr->glBindProgramPipeline(pipeline);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindSampler(QOpenGLExtraFunctions* this_ptr, GLuint unit, GLuint sampler) {
  this_ptr->glBindSampler(unit, sampler);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindTransformFeedback(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint id) {
  this_ptr->glBindTransformFeedback(target, id);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindVertexArray(QOpenGLExtraFunctions* this_ptr, GLuint array) {
  this_ptr->glBindVertexArray(array);
}

void qt_gui_c_QOpenGLExtraFunctions_glBindVertexBuffer(QOpenGLExtraFunctions* this_ptr, GLuint bindingindex, GLuint buffer, long offset, int stride) {
  this_ptr->glBindVertexBuffer(bindingindex, buffer, offset, stride);
}

void qt_gui_c_QOpenGLExtraFunctions_glBlitFramebuffer(QOpenGLExtraFunctions* this_ptr, GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, unsigned int mask, unsigned int filter) {
  this_ptr->glBlitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
}

void qt_gui_c_QOpenGLExtraFunctions_glClearBufferfi(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, float depth, GLint stencil) {
  this_ptr->glClearBufferfi(buffer, drawbuffer, depth, stencil);
}

void qt_gui_c_QOpenGLExtraFunctions_glClearBufferfv(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, const float* value) {
  this_ptr->glClearBufferfv(buffer, drawbuffer, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glClearBufferiv(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, const GLint* value) {
  this_ptr->glClearBufferiv(buffer, drawbuffer, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glClearBufferuiv(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, const GLuint* value) {
  this_ptr->glClearBufferuiv(buffer, drawbuffer, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glCompressedTexImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, unsigned int internalformat, int width, int height, int depth, GLint border, int imageSize, const void* data) {
  this_ptr->glCompressedTexImage3D(target, level, internalformat, width, height, depth, border, imageSize, data);
}

void qt_gui_c_QOpenGLExtraFunctions_glCompressedTexSubImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, int width, int height, int depth, unsigned int format, int imageSize, const void* data) {
  this_ptr->glCompressedTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data);
}

void qt_gui_c_QOpenGLExtraFunctions_glCopyBufferSubData(QOpenGLExtraFunctions* this_ptr, unsigned int readTarget, unsigned int writeTarget, long readOffset, long writeOffset, long size) {
  this_ptr->glCopyBufferSubData(readTarget, writeTarget, readOffset, writeOffset, size);
}

void qt_gui_c_QOpenGLExtraFunctions_glCopyTexSubImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLint x, GLint y, int width, int height) {
  this_ptr->glCopyTexSubImage3D(target, level, xoffset, yoffset, zoffset, x, y, width, height);
}

GLuint qt_gui_c_QOpenGLExtraFunctions_glCreateShaderProgramv(QOpenGLExtraFunctions* this_ptr, unsigned int type, int count, const char* const * strings) {
  return this_ptr->glCreateShaderProgramv(type, count, strings);
}

void qt_gui_c_QOpenGLExtraFunctions_glDeleteProgramPipelines(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* pipelines) {
  this_ptr->glDeleteProgramPipelines(n, pipelines);
}

void qt_gui_c_QOpenGLExtraFunctions_glDeleteQueries(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* ids) {
  this_ptr->glDeleteQueries(n, ids);
}

void qt_gui_c_QOpenGLExtraFunctions_glDeleteSamplers(QOpenGLExtraFunctions* this_ptr, int count, const GLuint* samplers) {
  this_ptr->glDeleteSamplers(count, samplers);
}

void qt_gui_c_QOpenGLExtraFunctions_glDeleteTransformFeedbacks(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* ids) {
  this_ptr->glDeleteTransformFeedbacks(n, ids);
}

void qt_gui_c_QOpenGLExtraFunctions_glDeleteVertexArrays(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* arrays) {
  this_ptr->glDeleteVertexArrays(n, arrays);
}

void qt_gui_c_QOpenGLExtraFunctions_glDispatchCompute(QOpenGLExtraFunctions* this_ptr, GLuint num_groups_x, GLuint num_groups_y, GLuint num_groups_z) {
  this_ptr->glDispatchCompute(num_groups_x, num_groups_y, num_groups_z);
}

void qt_gui_c_QOpenGLExtraFunctions_glDispatchComputeIndirect(QOpenGLExtraFunctions* this_ptr, long indirect) {
  this_ptr->glDispatchComputeIndirect(indirect);
}

void qt_gui_c_QOpenGLExtraFunctions_glDrawArraysIndirect(QOpenGLExtraFunctions* this_ptr, unsigned int mode, const void* indirect) {
  this_ptr->glDrawArraysIndirect(mode, indirect);
}

void qt_gui_c_QOpenGLExtraFunctions_glDrawArraysInstanced(QOpenGLExtraFunctions* this_ptr, unsigned int mode, GLint first, int count, int instancecount) {
  this_ptr->glDrawArraysInstanced(mode, first, count, instancecount);
}

void qt_gui_c_QOpenGLExtraFunctions_glDrawBuffers(QOpenGLExtraFunctions* this_ptr, int n, const unsigned int* bufs) {
  this_ptr->glDrawBuffers(n, bufs);
}

void qt_gui_c_QOpenGLExtraFunctions_glDrawElementsIndirect(QOpenGLExtraFunctions* this_ptr, unsigned int mode, unsigned int type, const void* indirect) {
  this_ptr->glDrawElementsIndirect(mode, type, indirect);
}

void qt_gui_c_QOpenGLExtraFunctions_glDrawElementsInstanced(QOpenGLExtraFunctions* this_ptr, unsigned int mode, int count, unsigned int type, const void* indices, int instancecount) {
  this_ptr->glDrawElementsInstanced(mode, count, type, indices, instancecount);
}

void qt_gui_c_QOpenGLExtraFunctions_glDrawRangeElements(QOpenGLExtraFunctions* this_ptr, unsigned int mode, GLuint start, GLuint end, int count, unsigned int type, const void* indices) {
  this_ptr->glDrawRangeElements(mode, start, end, count, type, indices);
}

void qt_gui_c_QOpenGLExtraFunctions_glEndQuery(QOpenGLExtraFunctions* this_ptr, unsigned int target) {
  this_ptr->glEndQuery(target);
}

void qt_gui_c_QOpenGLExtraFunctions_glEndTransformFeedback(QOpenGLExtraFunctions* this_ptr) {
  this_ptr->glEndTransformFeedback();
}

void qt_gui_c_QOpenGLExtraFunctions_glFlushMappedBufferRange(QOpenGLExtraFunctions* this_ptr, unsigned int target, long offset, long length) {
  this_ptr->glFlushMappedBufferRange(target, offset, length);
}

void qt_gui_c_QOpenGLExtraFunctions_glFramebufferParameteri(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint param) {
  this_ptr->glFramebufferParameteri(target, pname, param);
}

void qt_gui_c_QOpenGLExtraFunctions_glFramebufferTextureLayer(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int attachment, GLuint texture, GLint level, GLint layer) {
  this_ptr->glFramebufferTextureLayer(target, attachment, texture, level, layer);
}

void qt_gui_c_QOpenGLExtraFunctions_glGenProgramPipelines(QOpenGLExtraFunctions* this_ptr, int n, GLuint* pipelines) {
  this_ptr->glGenProgramPipelines(n, pipelines);
}

void qt_gui_c_QOpenGLExtraFunctions_glGenQueries(QOpenGLExtraFunctions* this_ptr, int n, GLuint* ids) {
  this_ptr->glGenQueries(n, ids);
}

void qt_gui_c_QOpenGLExtraFunctions_glGenSamplers(QOpenGLExtraFunctions* this_ptr, int count, GLuint* samplers) {
  this_ptr->glGenSamplers(count, samplers);
}

void qt_gui_c_QOpenGLExtraFunctions_glGenTransformFeedbacks(QOpenGLExtraFunctions* this_ptr, int n, GLuint* ids) {
  this_ptr->glGenTransformFeedbacks(n, ids);
}

void qt_gui_c_QOpenGLExtraFunctions_glGenVertexArrays(QOpenGLExtraFunctions* this_ptr, int n, GLuint* arrays) {
  this_ptr->glGenVertexArrays(n, arrays);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformBlockName(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint uniformBlockIndex, int bufSize, int* length, char* uniformBlockName) {
  this_ptr->glGetActiveUniformBlockName(program, uniformBlockIndex, bufSize, length, uniformBlockName);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformBlockiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint uniformBlockIndex, unsigned int pname, GLint* params) {
  this_ptr->glGetActiveUniformBlockiv(program, uniformBlockIndex, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformsiv(QOpenGLExtraFunctions* this_ptr, GLuint program, int uniformCount, const GLuint* uniformIndices, unsigned int pname, GLint* params) {
  this_ptr->glGetActiveUniformsiv(program, uniformCount, uniformIndices, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetBooleani_v(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, unsigned char* data) {
  this_ptr->glGetBooleani_v(target, index, data);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetBufferParameteri64v(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint64* params) {
  this_ptr->glGetBufferParameteri64v(target, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetBufferPointerv(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, void** params) {
  this_ptr->glGetBufferPointerv(target, pname, params);
}

GLint qt_gui_c_QOpenGLExtraFunctions_glGetFragDataLocation(QOpenGLExtraFunctions* this_ptr, GLuint program, const char* name) {
  return this_ptr->glGetFragDataLocation(program, name);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetFramebufferParameteriv(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint* params) {
  this_ptr->glGetFramebufferParameteriv(target, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetInteger64i_v(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLint64* data) {
  this_ptr->glGetInteger64i_v(target, index, data);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetInteger64v(QOpenGLExtraFunctions* this_ptr, unsigned int pname, GLint64* data) {
  this_ptr->glGetInteger64v(pname, data);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetIntegeri_v(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLint* data) {
  this_ptr->glGetIntegeri_v(target, index, data);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetInternalformativ(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int internalformat, unsigned int pname, int bufSize, GLint* params) {
  this_ptr->glGetInternalformativ(target, internalformat, pname, bufSize, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetMultisamplefv(QOpenGLExtraFunctions* this_ptr, unsigned int pname, GLuint index, float* val) {
  this_ptr->glGetMultisamplefv(pname, index, val);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetProgramBinary(QOpenGLExtraFunctions* this_ptr, GLuint program, int bufSize, int* length, unsigned int* binaryFormat, void* binary) {
  this_ptr->glGetProgramBinary(program, bufSize, length, binaryFormat, binary);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetProgramInterfaceiv(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, unsigned int pname, GLint* params) {
  this_ptr->glGetProgramInterfaceiv(program, programInterface, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetProgramPipelineInfoLog(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, int bufSize, int* length, char* infoLog) {
  this_ptr->glGetProgramPipelineInfoLog(pipeline, bufSize, length, infoLog);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetProgramPipelineiv(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, unsigned int pname, GLint* params) {
  this_ptr->glGetProgramPipelineiv(pipeline, pname, params);
}

GLuint qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceIndex(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, const char* name) {
  return this_ptr->glGetProgramResourceIndex(program, programInterface, name);
}

GLint qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceLocation(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, const char* name) {
  return this_ptr->glGetProgramResourceLocation(program, programInterface, name);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceName(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, GLuint index, int bufSize, int* length, char* name) {
  this_ptr->glGetProgramResourceName(program, programInterface, index, bufSize, length, name);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceiv(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, GLuint index, int propCount, const unsigned int* props, int bufSize, int* length, GLint* params) {
  this_ptr->glGetProgramResourceiv(program, programInterface, index, propCount, props, bufSize, length, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetQueryObjectuiv(QOpenGLExtraFunctions* this_ptr, GLuint id, unsigned int pname, GLuint* params) {
  this_ptr->glGetQueryObjectuiv(id, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetQueryiv(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint* params) {
  this_ptr->glGetQueryiv(target, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetSamplerParameterfv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, float* params) {
  this_ptr->glGetSamplerParameterfv(sampler, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetSamplerParameteriv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, GLint* params) {
  this_ptr->glGetSamplerParameteriv(sampler, pname, params);
}

const GLubyte* qt_gui_c_QOpenGLExtraFunctions_glGetStringi(QOpenGLExtraFunctions* this_ptr, unsigned int name, GLuint index) {
  return this_ptr->glGetStringi(name, index);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetTexLevelParameterfv(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, unsigned int pname, float* params) {
  this_ptr->glGetTexLevelParameterfv(target, level, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetTexLevelParameteriv(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, unsigned int pname, GLint* params) {
  this_ptr->glGetTexLevelParameteriv(target, level, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetTransformFeedbackVarying(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint index, int bufSize, int* length, int* size, unsigned int* type, char* name) {
  this_ptr->glGetTransformFeedbackVarying(program, index, bufSize, length, size, type, name);
}

GLuint qt_gui_c_QOpenGLExtraFunctions_glGetUniformBlockIndex(QOpenGLExtraFunctions* this_ptr, GLuint program, const char* uniformBlockName) {
  return this_ptr->glGetUniformBlockIndex(program, uniformBlockName);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetUniformIndices(QOpenGLExtraFunctions* this_ptr, GLuint program, int uniformCount, const char* const * uniformNames, GLuint* uniformIndices) {
  this_ptr->glGetUniformIndices(program, uniformCount, uniformNames, uniformIndices);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetUniformuiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint* params) {
  this_ptr->glGetUniformuiv(program, location, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetVertexAttribIiv(QOpenGLExtraFunctions* this_ptr, GLuint index, unsigned int pname, GLint* params) {
  this_ptr->glGetVertexAttribIiv(index, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glGetVertexAttribIuiv(QOpenGLExtraFunctions* this_ptr, GLuint index, unsigned int pname, GLuint* params) {
  this_ptr->glGetVertexAttribIuiv(index, pname, params);
}

void qt_gui_c_QOpenGLExtraFunctions_glInvalidateFramebuffer(QOpenGLExtraFunctions* this_ptr, unsigned int target, int numAttachments, const unsigned int* attachments) {
  this_ptr->glInvalidateFramebuffer(target, numAttachments, attachments);
}

void qt_gui_c_QOpenGLExtraFunctions_glInvalidateSubFramebuffer(QOpenGLExtraFunctions* this_ptr, unsigned int target, int numAttachments, const unsigned int* attachments, GLint x, GLint y, int width, int height) {
  this_ptr->glInvalidateSubFramebuffer(target, numAttachments, attachments, x, y, width, height);
}

unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsProgramPipeline(QOpenGLExtraFunctions* this_ptr, GLuint pipeline) {
  return this_ptr->glIsProgramPipeline(pipeline);
}

unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsQuery(QOpenGLExtraFunctions* this_ptr, GLuint id) {
  return this_ptr->glIsQuery(id);
}

unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsSampler(QOpenGLExtraFunctions* this_ptr, GLuint sampler) {
  return this_ptr->glIsSampler(sampler);
}

unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsTransformFeedback(QOpenGLExtraFunctions* this_ptr, GLuint id) {
  return this_ptr->glIsTransformFeedback(id);
}

unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsVertexArray(QOpenGLExtraFunctions* this_ptr, GLuint array) {
  return this_ptr->glIsVertexArray(array);
}

void* qt_gui_c_QOpenGLExtraFunctions_glMapBufferRange(QOpenGLExtraFunctions* this_ptr, unsigned int target, long offset, long length, unsigned int access) {
  return this_ptr->glMapBufferRange(target, offset, length, access);
}

void qt_gui_c_QOpenGLExtraFunctions_glMemoryBarrier(QOpenGLExtraFunctions* this_ptr, unsigned int barriers) {
  this_ptr->glMemoryBarrier(barriers);
}

void qt_gui_c_QOpenGLExtraFunctions_glMemoryBarrierByRegion(QOpenGLExtraFunctions* this_ptr, unsigned int barriers) {
  this_ptr->glMemoryBarrierByRegion(barriers);
}

void qt_gui_c_QOpenGLExtraFunctions_glPauseTransformFeedback(QOpenGLExtraFunctions* this_ptr) {
  this_ptr->glPauseTransformFeedback();
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramBinary(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int binaryFormat, const void* binary, int length) {
  this_ptr->glProgramBinary(program, binaryFormat, binary, length);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramParameteri(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int pname, GLint value) {
  this_ptr->glProgramParameteri(program, pname, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0) {
  this_ptr->glProgramUniform1f(program, location, v0);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value) {
  this_ptr->glProgramUniform1fv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0) {
  this_ptr->glProgramUniform1i(program, location, v0);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value) {
  this_ptr->glProgramUniform1iv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0) {
  this_ptr->glProgramUniform1ui(program, location, v0);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value) {
  this_ptr->glProgramUniform1uiv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0, float v1) {
  this_ptr->glProgramUniform2f(program, location, v0, v1);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value) {
  this_ptr->glProgramUniform2fv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0, GLint v1) {
  this_ptr->glProgramUniform2i(program, location, v0, v1);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value) {
  this_ptr->glProgramUniform2iv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0, GLuint v1) {
  this_ptr->glProgramUniform2ui(program, location, v0, v1);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value) {
  this_ptr->glProgramUniform2uiv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0, float v1, float v2) {
  this_ptr->glProgramUniform3f(program, location, v0, v1, v2);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value) {
  this_ptr->glProgramUniform3fv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0, GLint v1, GLint v2) {
  this_ptr->glProgramUniform3i(program, location, v0, v1, v2);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value) {
  this_ptr->glProgramUniform3iv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2) {
  this_ptr->glProgramUniform3ui(program, location, v0, v1, v2);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value) {
  this_ptr->glProgramUniform3uiv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0, float v1, float v2, float v3) {
  this_ptr->glProgramUniform4f(program, location, v0, v1, v2, v3);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value) {
  this_ptr->glProgramUniform4fv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0, GLint v1, GLint v2, GLint v3) {
  this_ptr->glProgramUniform4i(program, location, v0, v1, v2, v3);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value) {
  this_ptr->glProgramUniform4iv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3) {
  this_ptr->glProgramUniform4ui(program, location, v0, v1, v2, v3);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value) {
  this_ptr->glProgramUniform4uiv(program, location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix2fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2x3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix2x3fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2x4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix2x4fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix3fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3x2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix3x2fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3x4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix3x4fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix4fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4x2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix4x2fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4x3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glProgramUniformMatrix4x3fv(program, location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glReadBuffer(QOpenGLExtraFunctions* this_ptr, unsigned int mode) {
  this_ptr->glReadBuffer(mode);
}

void qt_gui_c_QOpenGLExtraFunctions_glRenderbufferStorageMultisample(QOpenGLExtraFunctions* this_ptr, unsigned int target, int samples, unsigned int internalformat, int width, int height) {
  this_ptr->glRenderbufferStorageMultisample(target, samples, internalformat, width, height);
}

void qt_gui_c_QOpenGLExtraFunctions_glResumeTransformFeedback(QOpenGLExtraFunctions* this_ptr) {
  this_ptr->glResumeTransformFeedback();
}

void qt_gui_c_QOpenGLExtraFunctions_glSampleMaski(QOpenGLExtraFunctions* this_ptr, GLuint maskNumber, unsigned int mask) {
  this_ptr->glSampleMaski(maskNumber, mask);
}

void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameterf(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, float param) {
  this_ptr->glSamplerParameterf(sampler, pname, param);
}

void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameterfv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, const float* param) {
  this_ptr->glSamplerParameterfv(sampler, pname, param);
}

void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameteri(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, GLint param) {
  this_ptr->glSamplerParameteri(sampler, pname, param);
}

void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameteriv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, const GLint* param) {
  this_ptr->glSamplerParameteriv(sampler, pname, param);
}

void qt_gui_c_QOpenGLExtraFunctions_glTexImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint internalformat, int width, int height, int depth, GLint border, unsigned int format, unsigned int type, const void* pixels) {
  this_ptr->glTexImage3D(target, level, internalformat, width, height, depth, border, format, type, pixels);
}

void qt_gui_c_QOpenGLExtraFunctions_glTexStorage2D(QOpenGLExtraFunctions* this_ptr, unsigned int target, int levels, unsigned int internalformat, int width, int height) {
  this_ptr->glTexStorage2D(target, levels, internalformat, width, height);
}

void qt_gui_c_QOpenGLExtraFunctions_glTexStorage2DMultisample(QOpenGLExtraFunctions* this_ptr, unsigned int target, int samples, unsigned int internalformat, int width, int height, unsigned char fixedsamplelocations) {
  this_ptr->glTexStorage2DMultisample(target, samples, internalformat, width, height, fixedsamplelocations);
}

void qt_gui_c_QOpenGLExtraFunctions_glTexStorage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, int levels, unsigned int internalformat, int width, int height, int depth) {
  this_ptr->glTexStorage3D(target, levels, internalformat, width, height, depth);
}

void qt_gui_c_QOpenGLExtraFunctions_glTexSubImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, int width, int height, int depth, unsigned int format, unsigned int type, const void* pixels) {
  this_ptr->glTexSubImage3D(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type, pixels);
}

void qt_gui_c_QOpenGLExtraFunctions_glTransformFeedbackVaryings(QOpenGLExtraFunctions* this_ptr, GLuint program, int count, const char* const * varyings, unsigned int bufferMode) {
  this_ptr->glTransformFeedbackVaryings(program, count, varyings, bufferMode);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform1ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0) {
  this_ptr->glUniform1ui(location, v0);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform1uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value) {
  this_ptr->glUniform1uiv(location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform2ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0, GLuint v1) {
  this_ptr->glUniform2ui(location, v0, v1);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform2uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value) {
  this_ptr->glUniform2uiv(location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform3ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0, GLuint v1, GLuint v2) {
  this_ptr->glUniform3ui(location, v0, v1, v2);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform3uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value) {
  this_ptr->glUniform3uiv(location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform4ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3) {
  this_ptr->glUniform4ui(location, v0, v1, v2, v3);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniform4uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value) {
  this_ptr->glUniform4uiv(location, count, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniformBlockBinding(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint uniformBlockIndex, GLuint uniformBlockBinding) {
  this_ptr->glUniformBlockBinding(program, uniformBlockIndex, uniformBlockBinding);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix2x3fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix2x3fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix2x4fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix2x4fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix3x2fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix3x2fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix3x4fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix3x4fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix4x2fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix4x2fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix4x3fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix4x3fv(location, count, transpose, value);
}

unsigned char qt_gui_c_QOpenGLExtraFunctions_glUnmapBuffer(QOpenGLExtraFunctions* this_ptr, unsigned int target) {
  return this_ptr->glUnmapBuffer(target);
}

void qt_gui_c_QOpenGLExtraFunctions_glUseProgramStages(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, unsigned int stages, GLuint program) {
  this_ptr->glUseProgramStages(pipeline, stages, program);
}

void qt_gui_c_QOpenGLExtraFunctions_glValidateProgramPipeline(QOpenGLExtraFunctions* this_ptr, GLuint pipeline) {
  this_ptr->glValidateProgramPipeline(pipeline);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribBinding(QOpenGLExtraFunctions* this_ptr, GLuint attribindex, GLuint bindingindex) {
  this_ptr->glVertexAttribBinding(attribindex, bindingindex);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribDivisor(QOpenGLExtraFunctions* this_ptr, GLuint index, GLuint divisor) {
  this_ptr->glVertexAttribDivisor(index, divisor);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribFormat(QOpenGLExtraFunctions* this_ptr, GLuint attribindex, GLint size, unsigned int type, unsigned char normalized, GLuint relativeoffset) {
  this_ptr->glVertexAttribFormat(attribindex, size, type, normalized, relativeoffset);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4i(QOpenGLExtraFunctions* this_ptr, GLuint index, GLint x, GLint y, GLint z, GLint w) {
  this_ptr->glVertexAttribI4i(index, x, y, z, w);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4iv(QOpenGLExtraFunctions* this_ptr, GLuint index, const GLint* v) {
  this_ptr->glVertexAttribI4iv(index, v);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4ui(QOpenGLExtraFunctions* this_ptr, GLuint index, GLuint x, GLuint y, GLuint z, GLuint w) {
  this_ptr->glVertexAttribI4ui(index, x, y, z, w);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4uiv(QOpenGLExtraFunctions* this_ptr, GLuint index, const GLuint* v) {
  this_ptr->glVertexAttribI4uiv(index, v);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribIFormat(QOpenGLExtraFunctions* this_ptr, GLuint attribindex, GLint size, unsigned int type, GLuint relativeoffset) {
  this_ptr->glVertexAttribIFormat(attribindex, size, type, relativeoffset);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribIPointer(QOpenGLExtraFunctions* this_ptr, GLuint index, GLint size, unsigned int type, int stride, const void* pointer) {
  this_ptr->glVertexAttribIPointer(index, size, type, stride, pointer);
}

void qt_gui_c_QOpenGLExtraFunctions_glVertexBindingDivisor(QOpenGLExtraFunctions* this_ptr, GLuint bindingindex, GLuint divisor) {
  this_ptr->glVertexBindingDivisor(bindingindex, divisor);
}

QOpenGLExtraFunctions* qt_gui_c_QOpenGLExtraFunctions_new_context(QOpenGLContext* context) {
  return new QOpenGLExtraFunctions(context);
}

QOpenGLExtraFunctions* qt_gui_c_QOpenGLExtraFunctions_new_no_args() {
  return new QOpenGLExtraFunctions();
}

