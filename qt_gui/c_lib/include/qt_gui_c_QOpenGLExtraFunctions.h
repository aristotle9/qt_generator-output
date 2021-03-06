#ifndef QT_GUI_C_QOPENGLEXTRAFUNCTIONS_H
#define QT_GUI_C_QOPENGLEXTRAFUNCTIONS_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QOpenGLExtraFunctions* qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLExtraFunctions_ptr(QOpenGLFunctions* ptr);
QT_GUI_C_EXPORT QOpenGLFunctions* qt_gui_c_QOpenGLExtraFunctions_G_static_cast_QOpenGLFunctions_ptr(QOpenGLExtraFunctions* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_delete(QOpenGLExtraFunctions* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glActiveShaderProgram(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, GLuint program);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBeginQuery(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint id);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBeginTransformFeedback(QOpenGLExtraFunctions* this_ptr, unsigned int primitiveMode);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindBufferBase(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLuint buffer);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindBufferRange(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLuint buffer, long offset, long size);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindImageTexture(QOpenGLExtraFunctions* this_ptr, GLuint unit, GLuint texture, GLint level, unsigned char layered, GLint layer, unsigned int access, unsigned int format);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindProgramPipeline(QOpenGLExtraFunctions* this_ptr, GLuint pipeline);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindSampler(QOpenGLExtraFunctions* this_ptr, GLuint unit, GLuint sampler);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindTransformFeedback(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint id);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindVertexArray(QOpenGLExtraFunctions* this_ptr, GLuint array);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBindVertexBuffer(QOpenGLExtraFunctions* this_ptr, GLuint bindingindex, GLuint buffer, long offset, int stride);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glBlitFramebuffer(QOpenGLExtraFunctions* this_ptr, GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, unsigned int mask, unsigned int filter);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glClearBufferfi(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, float depth, GLint stencil);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glClearBufferfv(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glClearBufferiv(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, const GLint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glClearBufferuiv(QOpenGLExtraFunctions* this_ptr, unsigned int buffer, GLint drawbuffer, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glCompressedTexImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, unsigned int internalformat, int width, int height, int depth, GLint border, int imageSize, const void* data);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glCompressedTexSubImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, int width, int height, int depth, unsigned int format, int imageSize, const void* data);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glCopyBufferSubData(QOpenGLExtraFunctions* this_ptr, unsigned int readTarget, unsigned int writeTarget, long readOffset, long writeOffset, long size);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glCopyTexSubImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLint x, GLint y, int width, int height);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLExtraFunctions_glCreateShaderProgramv(QOpenGLExtraFunctions* this_ptr, unsigned int type, int count, const char* const * strings);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDeleteProgramPipelines(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* pipelines);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDeleteQueries(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* ids);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDeleteSamplers(QOpenGLExtraFunctions* this_ptr, int count, const GLuint* samplers);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDeleteTransformFeedbacks(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* ids);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDeleteVertexArrays(QOpenGLExtraFunctions* this_ptr, int n, const GLuint* arrays);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDispatchCompute(QOpenGLExtraFunctions* this_ptr, GLuint num_groups_x, GLuint num_groups_y, GLuint num_groups_z);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDispatchComputeIndirect(QOpenGLExtraFunctions* this_ptr, long indirect);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDrawArraysIndirect(QOpenGLExtraFunctions* this_ptr, unsigned int mode, const void* indirect);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDrawArraysInstanced(QOpenGLExtraFunctions* this_ptr, unsigned int mode, GLint first, int count, int instancecount);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDrawBuffers(QOpenGLExtraFunctions* this_ptr, int n, const unsigned int* bufs);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDrawElementsIndirect(QOpenGLExtraFunctions* this_ptr, unsigned int mode, unsigned int type, const void* indirect);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDrawElementsInstanced(QOpenGLExtraFunctions* this_ptr, unsigned int mode, int count, unsigned int type, const void* indices, int instancecount);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glDrawRangeElements(QOpenGLExtraFunctions* this_ptr, unsigned int mode, GLuint start, GLuint end, int count, unsigned int type, const void* indices);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glEndQuery(QOpenGLExtraFunctions* this_ptr, unsigned int target);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glEndTransformFeedback(QOpenGLExtraFunctions* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glFlushMappedBufferRange(QOpenGLExtraFunctions* this_ptr, unsigned int target, long offset, long length);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glFramebufferParameteri(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint param);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glFramebufferTextureLayer(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int attachment, GLuint texture, GLint level, GLint layer);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGenProgramPipelines(QOpenGLExtraFunctions* this_ptr, int n, GLuint* pipelines);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGenQueries(QOpenGLExtraFunctions* this_ptr, int n, GLuint* ids);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGenSamplers(QOpenGLExtraFunctions* this_ptr, int count, GLuint* samplers);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGenTransformFeedbacks(QOpenGLExtraFunctions* this_ptr, int n, GLuint* ids);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGenVertexArrays(QOpenGLExtraFunctions* this_ptr, int n, GLuint* arrays);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformBlockName(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint uniformBlockIndex, int bufSize, int* length, char* uniformBlockName);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformBlockiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint uniformBlockIndex, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetActiveUniformsiv(QOpenGLExtraFunctions* this_ptr, GLuint program, int uniformCount, const GLuint* uniformIndices, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetBooleani_v(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, unsigned char* data);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetBufferParameteri64v(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint64* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetBufferPointerv(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, void** params);
QT_GUI_C_EXPORT GLint qt_gui_c_QOpenGLExtraFunctions_glGetFragDataLocation(QOpenGLExtraFunctions* this_ptr, GLuint program, const char* name);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetFramebufferParameteriv(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetInteger64i_v(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLint64* data);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetInteger64v(QOpenGLExtraFunctions* this_ptr, unsigned int pname, GLint64* data);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetIntegeri_v(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLuint index, GLint* data);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetInternalformativ(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int internalformat, unsigned int pname, int bufSize, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetMultisamplefv(QOpenGLExtraFunctions* this_ptr, unsigned int pname, GLuint index, float* val);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetProgramBinary(QOpenGLExtraFunctions* this_ptr, GLuint program, int bufSize, int* length, unsigned int* binaryFormat, void* binary);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetProgramInterfaceiv(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetProgramPipelineInfoLog(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, int bufSize, int* length, char* infoLog);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetProgramPipelineiv(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceIndex(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, const char* name);
QT_GUI_C_EXPORT GLint qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceLocation(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, const char* name);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceName(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, GLuint index, int bufSize, int* length, char* name);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetProgramResourceiv(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int programInterface, GLuint index, int propCount, const unsigned int* props, int bufSize, int* length, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetQueryObjectuiv(QOpenGLExtraFunctions* this_ptr, GLuint id, unsigned int pname, GLuint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetQueryiv(QOpenGLExtraFunctions* this_ptr, unsigned int target, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetSamplerParameterfv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, float* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetSamplerParameteriv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT const GLubyte* qt_gui_c_QOpenGLExtraFunctions_glGetStringi(QOpenGLExtraFunctions* this_ptr, unsigned int name, GLuint index);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetTexLevelParameterfv(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, unsigned int pname, float* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetTexLevelParameteriv(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetTransformFeedbackVarying(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint index, int bufSize, int* length, int* size, unsigned int* type, char* name);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLExtraFunctions_glGetUniformBlockIndex(QOpenGLExtraFunctions* this_ptr, GLuint program, const char* uniformBlockName);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetUniformIndices(QOpenGLExtraFunctions* this_ptr, GLuint program, int uniformCount, const char* const * uniformNames, GLuint* uniformIndices);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetUniformuiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetVertexAttribIiv(QOpenGLExtraFunctions* this_ptr, GLuint index, unsigned int pname, GLint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glGetVertexAttribIuiv(QOpenGLExtraFunctions* this_ptr, GLuint index, unsigned int pname, GLuint* params);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glInvalidateFramebuffer(QOpenGLExtraFunctions* this_ptr, unsigned int target, int numAttachments, const unsigned int* attachments);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glInvalidateSubFramebuffer(QOpenGLExtraFunctions* this_ptr, unsigned int target, int numAttachments, const unsigned int* attachments, GLint x, GLint y, int width, int height);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsProgramPipeline(QOpenGLExtraFunctions* this_ptr, GLuint pipeline);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsQuery(QOpenGLExtraFunctions* this_ptr, GLuint id);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsSampler(QOpenGLExtraFunctions* this_ptr, GLuint sampler);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsTransformFeedback(QOpenGLExtraFunctions* this_ptr, GLuint id);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QOpenGLExtraFunctions_glIsVertexArray(QOpenGLExtraFunctions* this_ptr, GLuint array);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLExtraFunctions_glMapBufferRange(QOpenGLExtraFunctions* this_ptr, unsigned int target, long offset, long length, unsigned int access);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glMemoryBarrier(QOpenGLExtraFunctions* this_ptr, unsigned int barriers);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glMemoryBarrierByRegion(QOpenGLExtraFunctions* this_ptr, unsigned int barriers);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glPauseTransformFeedback(QOpenGLExtraFunctions* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramBinary(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int binaryFormat, const void* binary, int length);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramParameteri(QOpenGLExtraFunctions* this_ptr, GLuint program, unsigned int pname, GLint value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform1uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0, float v1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0, GLint v1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0, GLuint v1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform2uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0, float v1, float v2);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0, GLint v1, GLint v2);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform3uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4f(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, float v0, float v1, float v2, float v3);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4i(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLint v0, GLint v1, GLint v2, GLint v3);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4iv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4ui(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniform4uiv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2x3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix2x4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3x2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix3x4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4x2fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glProgramUniformMatrix4x3fv(QOpenGLExtraFunctions* this_ptr, GLuint program, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glReadBuffer(QOpenGLExtraFunctions* this_ptr, unsigned int mode);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glRenderbufferStorageMultisample(QOpenGLExtraFunctions* this_ptr, unsigned int target, int samples, unsigned int internalformat, int width, int height);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glResumeTransformFeedback(QOpenGLExtraFunctions* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glSampleMaski(QOpenGLExtraFunctions* this_ptr, GLuint maskNumber, unsigned int mask);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameterf(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, float param);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameterfv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, const float* param);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameteri(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, GLint param);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glSamplerParameteriv(QOpenGLExtraFunctions* this_ptr, GLuint sampler, unsigned int pname, const GLint* param);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glTexImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint internalformat, int width, int height, int depth, GLint border, unsigned int format, unsigned int type, const void* pixels);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glTexStorage2D(QOpenGLExtraFunctions* this_ptr, unsigned int target, int levels, unsigned int internalformat, int width, int height);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glTexStorage2DMultisample(QOpenGLExtraFunctions* this_ptr, unsigned int target, int samples, unsigned int internalformat, int width, int height, unsigned char fixedsamplelocations);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glTexStorage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, int levels, unsigned int internalformat, int width, int height, int depth);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glTexSubImage3D(QOpenGLExtraFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, int width, int height, int depth, unsigned int format, unsigned int type, const void* pixels);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glTransformFeedbackVaryings(QOpenGLExtraFunctions* this_ptr, GLuint program, int count, const char* const * varyings, unsigned int bufferMode);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform1ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform1uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform2ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0, GLuint v1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform2uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform3ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0, GLuint v1, GLuint v2);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform3uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform4ui(QOpenGLExtraFunctions* this_ptr, GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniform4uiv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, const GLuint* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniformBlockBinding(QOpenGLExtraFunctions* this_ptr, GLuint program, GLuint uniformBlockIndex, GLuint uniformBlockBinding);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix2x3fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix2x4fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix3x2fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix3x4fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix4x2fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUniformMatrix4x3fv(QOpenGLExtraFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value);
QT_GUI_C_EXPORT unsigned char qt_gui_c_QOpenGLExtraFunctions_glUnmapBuffer(QOpenGLExtraFunctions* this_ptr, unsigned int target);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glUseProgramStages(QOpenGLExtraFunctions* this_ptr, GLuint pipeline, unsigned int stages, GLuint program);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glValidateProgramPipeline(QOpenGLExtraFunctions* this_ptr, GLuint pipeline);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribBinding(QOpenGLExtraFunctions* this_ptr, GLuint attribindex, GLuint bindingindex);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribDivisor(QOpenGLExtraFunctions* this_ptr, GLuint index, GLuint divisor);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribFormat(QOpenGLExtraFunctions* this_ptr, GLuint attribindex, GLint size, unsigned int type, unsigned char normalized, GLuint relativeoffset);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4i(QOpenGLExtraFunctions* this_ptr, GLuint index, GLint x, GLint y, GLint z, GLint w);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4iv(QOpenGLExtraFunctions* this_ptr, GLuint index, const GLint* v);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4ui(QOpenGLExtraFunctions* this_ptr, GLuint index, GLuint x, GLuint y, GLuint z, GLuint w);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribI4uiv(QOpenGLExtraFunctions* this_ptr, GLuint index, const GLuint* v);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribIFormat(QOpenGLExtraFunctions* this_ptr, GLuint attribindex, GLint size, unsigned int type, GLuint relativeoffset);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexAttribIPointer(QOpenGLExtraFunctions* this_ptr, GLuint index, GLint size, unsigned int type, int stride, const void* pointer);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLExtraFunctions_glVertexBindingDivisor(QOpenGLExtraFunctions* this_ptr, GLuint bindingindex, GLuint divisor);
QT_GUI_C_EXPORT QOpenGLExtraFunctions* qt_gui_c_QOpenGLExtraFunctions_new_context(QOpenGLContext* context);
QT_GUI_C_EXPORT QOpenGLExtraFunctions* qt_gui_c_QOpenGLExtraFunctions_new_no_args();

} // extern "C"

#endif // QT_GUI_C_QOPENGLEXTRAFUNCTIONS_H
