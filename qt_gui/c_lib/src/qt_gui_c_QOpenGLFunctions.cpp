#include "qt_gui_c_QOpenGLFunctions.h"

void qt_gui_c_QOpenGLFunctions_delete(QOpenGLFunctions* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QOpenGLFunctions_glActiveTexture(QOpenGLFunctions* this_ptr, unsigned int texture) {
  this_ptr->glActiveTexture(texture);
}

void qt_gui_c_QOpenGLFunctions_glAttachShader(QOpenGLFunctions* this_ptr, GLuint program, GLuint shader) {
  this_ptr->glAttachShader(program, shader);
}

void qt_gui_c_QOpenGLFunctions_glBindAttribLocation(QOpenGLFunctions* this_ptr, GLuint program, GLuint index, const char* name) {
  this_ptr->glBindAttribLocation(program, index, name);
}

void qt_gui_c_QOpenGLFunctions_glBindBuffer(QOpenGLFunctions* this_ptr, unsigned int target, GLuint buffer) {
  this_ptr->glBindBuffer(target, buffer);
}

void qt_gui_c_QOpenGLFunctions_glBindFramebuffer(QOpenGLFunctions* this_ptr, unsigned int target, GLuint framebuffer) {
  this_ptr->glBindFramebuffer(target, framebuffer);
}

void qt_gui_c_QOpenGLFunctions_glBindRenderbuffer(QOpenGLFunctions* this_ptr, unsigned int target, GLuint renderbuffer) {
  this_ptr->glBindRenderbuffer(target, renderbuffer);
}

void qt_gui_c_QOpenGLFunctions_glBindTexture(QOpenGLFunctions* this_ptr, unsigned int target, GLuint texture) {
  this_ptr->glBindTexture(target, texture);
}

void qt_gui_c_QOpenGLFunctions_glBlendColor(QOpenGLFunctions* this_ptr, float red, float green, float blue, float alpha) {
  this_ptr->glBlendColor(red, green, blue, alpha);
}

void qt_gui_c_QOpenGLFunctions_glBlendEquation(QOpenGLFunctions* this_ptr, unsigned int mode) {
  this_ptr->glBlendEquation(mode);
}

void qt_gui_c_QOpenGLFunctions_glBlendEquationSeparate(QOpenGLFunctions* this_ptr, unsigned int modeRGB, unsigned int modeAlpha) {
  this_ptr->glBlendEquationSeparate(modeRGB, modeAlpha);
}

void qt_gui_c_QOpenGLFunctions_glBlendFunc(QOpenGLFunctions* this_ptr, unsigned int sfactor, unsigned int dfactor) {
  this_ptr->glBlendFunc(sfactor, dfactor);
}

void qt_gui_c_QOpenGLFunctions_glBlendFuncSeparate(QOpenGLFunctions* this_ptr, unsigned int srcRGB, unsigned int dstRGB, unsigned int srcAlpha, unsigned int dstAlpha) {
  this_ptr->glBlendFuncSeparate(srcRGB, dstRGB, srcAlpha, dstAlpha);
}

void qt_gui_c_QOpenGLFunctions_glBufferData(QOpenGLFunctions* this_ptr, unsigned int target, long size, const void* data, unsigned int usage) {
  this_ptr->glBufferData(target, size, data, usage);
}

void qt_gui_c_QOpenGLFunctions_glBufferSubData(QOpenGLFunctions* this_ptr, unsigned int target, long offset, long size, const void* data) {
  this_ptr->glBufferSubData(target, offset, size, data);
}

unsigned int qt_gui_c_QOpenGLFunctions_glCheckFramebufferStatus(QOpenGLFunctions* this_ptr, unsigned int target) {
  return this_ptr->glCheckFramebufferStatus(target);
}

void qt_gui_c_QOpenGLFunctions_glClear(QOpenGLFunctions* this_ptr, unsigned int mask) {
  this_ptr->glClear(mask);
}

void qt_gui_c_QOpenGLFunctions_glClearColor(QOpenGLFunctions* this_ptr, float red, float green, float blue, float alpha) {
  this_ptr->glClearColor(red, green, blue, alpha);
}

void qt_gui_c_QOpenGLFunctions_glClearDepthf(QOpenGLFunctions* this_ptr, float depth) {
  this_ptr->glClearDepthf(depth);
}

void qt_gui_c_QOpenGLFunctions_glClearStencil(QOpenGLFunctions* this_ptr, GLint s) {
  this_ptr->glClearStencil(s);
}

void qt_gui_c_QOpenGLFunctions_glColorMask(QOpenGLFunctions* this_ptr, unsigned char red, unsigned char green, unsigned char blue, unsigned char alpha) {
  this_ptr->glColorMask(red, green, blue, alpha);
}

void qt_gui_c_QOpenGLFunctions_glCompileShader(QOpenGLFunctions* this_ptr, GLuint shader) {
  this_ptr->glCompileShader(shader);
}

void qt_gui_c_QOpenGLFunctions_glCompressedTexImage2D(QOpenGLFunctions* this_ptr, unsigned int target, GLint level, unsigned int internalformat, int width, int height, GLint border, int imageSize, const void* data) {
  this_ptr->glCompressedTexImage2D(target, level, internalformat, width, height, border, imageSize, data);
}

void qt_gui_c_QOpenGLFunctions_glCompressedTexSubImage2D(QOpenGLFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, int width, int height, unsigned int format, int imageSize, const void* data) {
  this_ptr->glCompressedTexSubImage2D(target, level, xoffset, yoffset, width, height, format, imageSize, data);
}

void qt_gui_c_QOpenGLFunctions_glCopyTexImage2D(QOpenGLFunctions* this_ptr, unsigned int target, GLint level, unsigned int internalformat, GLint x, GLint y, int width, int height, GLint border) {
  this_ptr->glCopyTexImage2D(target, level, internalformat, x, y, width, height, border);
}

void qt_gui_c_QOpenGLFunctions_glCopyTexSubImage2D(QOpenGLFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, int width, int height) {
  this_ptr->glCopyTexSubImage2D(target, level, xoffset, yoffset, x, y, width, height);
}

GLuint qt_gui_c_QOpenGLFunctions_glCreateProgram(QOpenGLFunctions* this_ptr) {
  return this_ptr->glCreateProgram();
}

GLuint qt_gui_c_QOpenGLFunctions_glCreateShader(QOpenGLFunctions* this_ptr, unsigned int type) {
  return this_ptr->glCreateShader(type);
}

void qt_gui_c_QOpenGLFunctions_glCullFace(QOpenGLFunctions* this_ptr, unsigned int mode) {
  this_ptr->glCullFace(mode);
}

void qt_gui_c_QOpenGLFunctions_glDeleteBuffers(QOpenGLFunctions* this_ptr, int n, const GLuint* buffers) {
  this_ptr->glDeleteBuffers(n, buffers);
}

void qt_gui_c_QOpenGLFunctions_glDeleteFramebuffers(QOpenGLFunctions* this_ptr, int n, const GLuint* framebuffers) {
  this_ptr->glDeleteFramebuffers(n, framebuffers);
}

void qt_gui_c_QOpenGLFunctions_glDeleteProgram(QOpenGLFunctions* this_ptr, GLuint program) {
  this_ptr->glDeleteProgram(program);
}

void qt_gui_c_QOpenGLFunctions_glDeleteRenderbuffers(QOpenGLFunctions* this_ptr, int n, const GLuint* renderbuffers) {
  this_ptr->glDeleteRenderbuffers(n, renderbuffers);
}

void qt_gui_c_QOpenGLFunctions_glDeleteShader(QOpenGLFunctions* this_ptr, GLuint shader) {
  this_ptr->glDeleteShader(shader);
}

void qt_gui_c_QOpenGLFunctions_glDeleteTextures(QOpenGLFunctions* this_ptr, int n, const GLuint* textures) {
  this_ptr->glDeleteTextures(n, textures);
}

void qt_gui_c_QOpenGLFunctions_glDepthFunc(QOpenGLFunctions* this_ptr, unsigned int func) {
  this_ptr->glDepthFunc(func);
}

void qt_gui_c_QOpenGLFunctions_glDepthMask(QOpenGLFunctions* this_ptr, unsigned char flag) {
  this_ptr->glDepthMask(flag);
}

void qt_gui_c_QOpenGLFunctions_glDepthRangef(QOpenGLFunctions* this_ptr, float zNear, float zFar) {
  this_ptr->glDepthRangef(zNear, zFar);
}

void qt_gui_c_QOpenGLFunctions_glDetachShader(QOpenGLFunctions* this_ptr, GLuint program, GLuint shader) {
  this_ptr->glDetachShader(program, shader);
}

void qt_gui_c_QOpenGLFunctions_glDisable(QOpenGLFunctions* this_ptr, unsigned int cap) {
  this_ptr->glDisable(cap);
}

void qt_gui_c_QOpenGLFunctions_glDisableVertexAttribArray(QOpenGLFunctions* this_ptr, GLuint index) {
  this_ptr->glDisableVertexAttribArray(index);
}

void qt_gui_c_QOpenGLFunctions_glDrawArrays(QOpenGLFunctions* this_ptr, unsigned int mode, GLint first, int count) {
  this_ptr->glDrawArrays(mode, first, count);
}

void qt_gui_c_QOpenGLFunctions_glDrawElements(QOpenGLFunctions* this_ptr, unsigned int mode, int count, unsigned int type, const void* indices) {
  this_ptr->glDrawElements(mode, count, type, indices);
}

void qt_gui_c_QOpenGLFunctions_glEnable(QOpenGLFunctions* this_ptr, unsigned int cap) {
  this_ptr->glEnable(cap);
}

void qt_gui_c_QOpenGLFunctions_glEnableVertexAttribArray(QOpenGLFunctions* this_ptr, GLuint index) {
  this_ptr->glEnableVertexAttribArray(index);
}

void qt_gui_c_QOpenGLFunctions_glFinish(QOpenGLFunctions* this_ptr) {
  this_ptr->glFinish();
}

void qt_gui_c_QOpenGLFunctions_glFlush(QOpenGLFunctions* this_ptr) {
  this_ptr->glFlush();
}

void qt_gui_c_QOpenGLFunctions_glFramebufferRenderbuffer(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int attachment, unsigned int renderbuffertarget, GLuint renderbuffer) {
  this_ptr->glFramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
}

void qt_gui_c_QOpenGLFunctions_glFramebufferTexture2D(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int attachment, unsigned int textarget, GLuint texture, GLint level) {
  this_ptr->glFramebufferTexture2D(target, attachment, textarget, texture, level);
}

void qt_gui_c_QOpenGLFunctions_glFrontFace(QOpenGLFunctions* this_ptr, unsigned int mode) {
  this_ptr->glFrontFace(mode);
}

void qt_gui_c_QOpenGLFunctions_glGenBuffers(QOpenGLFunctions* this_ptr, int n, GLuint* buffers) {
  this_ptr->glGenBuffers(n, buffers);
}

void qt_gui_c_QOpenGLFunctions_glGenFramebuffers(QOpenGLFunctions* this_ptr, int n, GLuint* framebuffers) {
  this_ptr->glGenFramebuffers(n, framebuffers);
}

void qt_gui_c_QOpenGLFunctions_glGenRenderbuffers(QOpenGLFunctions* this_ptr, int n, GLuint* renderbuffers) {
  this_ptr->glGenRenderbuffers(n, renderbuffers);
}

void qt_gui_c_QOpenGLFunctions_glGenTextures(QOpenGLFunctions* this_ptr, int n, GLuint* textures) {
  this_ptr->glGenTextures(n, textures);
}

void qt_gui_c_QOpenGLFunctions_glGenerateMipmap(QOpenGLFunctions* this_ptr, unsigned int target) {
  this_ptr->glGenerateMipmap(target);
}

void qt_gui_c_QOpenGLFunctions_glGetActiveAttrib(QOpenGLFunctions* this_ptr, GLuint program, GLuint index, int bufsize, int* length, GLint* size, unsigned int* type, char* name) {
  this_ptr->glGetActiveAttrib(program, index, bufsize, length, size, type, name);
}

void qt_gui_c_QOpenGLFunctions_glGetActiveUniform(QOpenGLFunctions* this_ptr, GLuint program, GLuint index, int bufsize, int* length, GLint* size, unsigned int* type, char* name) {
  this_ptr->glGetActiveUniform(program, index, bufsize, length, size, type, name);
}

void qt_gui_c_QOpenGLFunctions_glGetAttachedShaders(QOpenGLFunctions* this_ptr, GLuint program, int maxcount, int* count, GLuint* shaders) {
  this_ptr->glGetAttachedShaders(program, maxcount, count, shaders);
}

GLint qt_gui_c_QOpenGLFunctions_glGetAttribLocation(QOpenGLFunctions* this_ptr, GLuint program, const char* name) {
  return this_ptr->glGetAttribLocation(program, name);
}

void qt_gui_c_QOpenGLFunctions_glGetBooleanv(QOpenGLFunctions* this_ptr, unsigned int pname, unsigned char* params) {
  this_ptr->glGetBooleanv(pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetBufferParameteriv(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, GLint* params) {
  this_ptr->glGetBufferParameteriv(target, pname, params);
}

unsigned int qt_gui_c_QOpenGLFunctions_glGetError(QOpenGLFunctions* this_ptr) {
  return this_ptr->glGetError();
}

void qt_gui_c_QOpenGLFunctions_glGetFloatv(QOpenGLFunctions* this_ptr, unsigned int pname, float* params) {
  this_ptr->glGetFloatv(pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetFramebufferAttachmentParameteriv(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int attachment, unsigned int pname, GLint* params) {
  this_ptr->glGetFramebufferAttachmentParameteriv(target, attachment, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetIntegerv(QOpenGLFunctions* this_ptr, unsigned int pname, GLint* params) {
  this_ptr->glGetIntegerv(pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetProgramInfoLog(QOpenGLFunctions* this_ptr, GLuint program, int bufsize, int* length, char* infolog) {
  this_ptr->glGetProgramInfoLog(program, bufsize, length, infolog);
}

void qt_gui_c_QOpenGLFunctions_glGetProgramiv(QOpenGLFunctions* this_ptr, GLuint program, unsigned int pname, GLint* params) {
  this_ptr->glGetProgramiv(program, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetRenderbufferParameteriv(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, GLint* params) {
  this_ptr->glGetRenderbufferParameteriv(target, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetShaderInfoLog(QOpenGLFunctions* this_ptr, GLuint shader, int bufsize, int* length, char* infolog) {
  this_ptr->glGetShaderInfoLog(shader, bufsize, length, infolog);
}

void qt_gui_c_QOpenGLFunctions_glGetShaderPrecisionFormat(QOpenGLFunctions* this_ptr, unsigned int shadertype, unsigned int precisiontype, GLint* range, GLint* precision) {
  this_ptr->glGetShaderPrecisionFormat(shadertype, precisiontype, range, precision);
}

void qt_gui_c_QOpenGLFunctions_glGetShaderSource(QOpenGLFunctions* this_ptr, GLuint shader, int bufsize, int* length, char* source) {
  this_ptr->glGetShaderSource(shader, bufsize, length, source);
}

void qt_gui_c_QOpenGLFunctions_glGetShaderiv(QOpenGLFunctions* this_ptr, GLuint shader, unsigned int pname, GLint* params) {
  this_ptr->glGetShaderiv(shader, pname, params);
}

const GLubyte* qt_gui_c_QOpenGLFunctions_glGetString(QOpenGLFunctions* this_ptr, unsigned int name) {
  return this_ptr->glGetString(name);
}

void qt_gui_c_QOpenGLFunctions_glGetTexParameterfv(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, float* params) {
  this_ptr->glGetTexParameterfv(target, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetTexParameteriv(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, GLint* params) {
  this_ptr->glGetTexParameteriv(target, pname, params);
}

GLint qt_gui_c_QOpenGLFunctions_glGetUniformLocation(QOpenGLFunctions* this_ptr, GLuint program, const char* name) {
  return this_ptr->glGetUniformLocation(program, name);
}

void qt_gui_c_QOpenGLFunctions_glGetUniformfv(QOpenGLFunctions* this_ptr, GLuint program, GLint location, float* params) {
  this_ptr->glGetUniformfv(program, location, params);
}

void qt_gui_c_QOpenGLFunctions_glGetUniformiv(QOpenGLFunctions* this_ptr, GLuint program, GLint location, GLint* params) {
  this_ptr->glGetUniformiv(program, location, params);
}

void qt_gui_c_QOpenGLFunctions_glGetVertexAttribPointerv(QOpenGLFunctions* this_ptr, GLuint index, unsigned int pname, void** pointer) {
  this_ptr->glGetVertexAttribPointerv(index, pname, pointer);
}

void qt_gui_c_QOpenGLFunctions_glGetVertexAttribfv(QOpenGLFunctions* this_ptr, GLuint index, unsigned int pname, float* params) {
  this_ptr->glGetVertexAttribfv(index, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glGetVertexAttribiv(QOpenGLFunctions* this_ptr, GLuint index, unsigned int pname, GLint* params) {
  this_ptr->glGetVertexAttribiv(index, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glHint(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int mode) {
  this_ptr->glHint(target, mode);
}

unsigned char qt_gui_c_QOpenGLFunctions_glIsBuffer(QOpenGLFunctions* this_ptr, GLuint buffer) {
  return this_ptr->glIsBuffer(buffer);
}

unsigned char qt_gui_c_QOpenGLFunctions_glIsEnabled(QOpenGLFunctions* this_ptr, unsigned int cap) {
  return this_ptr->glIsEnabled(cap);
}

unsigned char qt_gui_c_QOpenGLFunctions_glIsFramebuffer(QOpenGLFunctions* this_ptr, GLuint framebuffer) {
  return this_ptr->glIsFramebuffer(framebuffer);
}

unsigned char qt_gui_c_QOpenGLFunctions_glIsProgram(QOpenGLFunctions* this_ptr, GLuint program) {
  return this_ptr->glIsProgram(program);
}

unsigned char qt_gui_c_QOpenGLFunctions_glIsRenderbuffer(QOpenGLFunctions* this_ptr, GLuint renderbuffer) {
  return this_ptr->glIsRenderbuffer(renderbuffer);
}

unsigned char qt_gui_c_QOpenGLFunctions_glIsShader(QOpenGLFunctions* this_ptr, GLuint shader) {
  return this_ptr->glIsShader(shader);
}

unsigned char qt_gui_c_QOpenGLFunctions_glIsTexture(QOpenGLFunctions* this_ptr, GLuint texture) {
  return this_ptr->glIsTexture(texture);
}

void qt_gui_c_QOpenGLFunctions_glLineWidth(QOpenGLFunctions* this_ptr, float width) {
  this_ptr->glLineWidth(width);
}

void qt_gui_c_QOpenGLFunctions_glLinkProgram(QOpenGLFunctions* this_ptr, GLuint program) {
  this_ptr->glLinkProgram(program);
}

void qt_gui_c_QOpenGLFunctions_glPixelStorei(QOpenGLFunctions* this_ptr, unsigned int pname, GLint param) {
  this_ptr->glPixelStorei(pname, param);
}

void qt_gui_c_QOpenGLFunctions_glPolygonOffset(QOpenGLFunctions* this_ptr, float factor, float units) {
  this_ptr->glPolygonOffset(factor, units);
}

void qt_gui_c_QOpenGLFunctions_glReadPixels(QOpenGLFunctions* this_ptr, GLint x, GLint y, int width, int height, unsigned int format, unsigned int type, void* pixels) {
  this_ptr->glReadPixels(x, y, width, height, format, type, pixels);
}

void qt_gui_c_QOpenGLFunctions_glReleaseShaderCompiler(QOpenGLFunctions* this_ptr) {
  this_ptr->glReleaseShaderCompiler();
}

void qt_gui_c_QOpenGLFunctions_glRenderbufferStorage(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int internalformat, int width, int height) {
  this_ptr->glRenderbufferStorage(target, internalformat, width, height);
}

void qt_gui_c_QOpenGLFunctions_glSampleCoverage(QOpenGLFunctions* this_ptr, float value, unsigned char invert) {
  this_ptr->glSampleCoverage(value, invert);
}

void qt_gui_c_QOpenGLFunctions_glScissor(QOpenGLFunctions* this_ptr, GLint x, GLint y, int width, int height) {
  this_ptr->glScissor(x, y, width, height);
}

void qt_gui_c_QOpenGLFunctions_glShaderBinary(QOpenGLFunctions* this_ptr, GLint n, const GLuint* shaders, unsigned int binaryformat, const void* binary, GLint length) {
  this_ptr->glShaderBinary(n, shaders, binaryformat, binary, length);
}

void qt_gui_c_QOpenGLFunctions_glShaderSource(QOpenGLFunctions* this_ptr, GLuint shader, int count, const char** string, const GLint* length) {
  this_ptr->glShaderSource(shader, count, string, length);
}

void qt_gui_c_QOpenGLFunctions_glStencilFunc(QOpenGLFunctions* this_ptr, unsigned int func, GLint ref, GLuint mask) {
  this_ptr->glStencilFunc(func, ref, mask);
}

void qt_gui_c_QOpenGLFunctions_glStencilFuncSeparate(QOpenGLFunctions* this_ptr, unsigned int face, unsigned int func, GLint ref, GLuint mask) {
  this_ptr->glStencilFuncSeparate(face, func, ref, mask);
}

void qt_gui_c_QOpenGLFunctions_glStencilMask(QOpenGLFunctions* this_ptr, GLuint mask) {
  this_ptr->glStencilMask(mask);
}

void qt_gui_c_QOpenGLFunctions_glStencilMaskSeparate(QOpenGLFunctions* this_ptr, unsigned int face, GLuint mask) {
  this_ptr->glStencilMaskSeparate(face, mask);
}

void qt_gui_c_QOpenGLFunctions_glStencilOp(QOpenGLFunctions* this_ptr, unsigned int fail, unsigned int zfail, unsigned int zpass) {
  this_ptr->glStencilOp(fail, zfail, zpass);
}

void qt_gui_c_QOpenGLFunctions_glStencilOpSeparate(QOpenGLFunctions* this_ptr, unsigned int face, unsigned int fail, unsigned int zfail, unsigned int zpass) {
  this_ptr->glStencilOpSeparate(face, fail, zfail, zpass);
}

void qt_gui_c_QOpenGLFunctions_glTexImage2D(QOpenGLFunctions* this_ptr, unsigned int target, GLint level, GLint internalformat, int width, int height, GLint border, unsigned int format, unsigned int type, const void* pixels) {
  this_ptr->glTexImage2D(target, level, internalformat, width, height, border, format, type, pixels);
}

void qt_gui_c_QOpenGLFunctions_glTexParameterf(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, float param) {
  this_ptr->glTexParameterf(target, pname, param);
}

void qt_gui_c_QOpenGLFunctions_glTexParameterfv(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, const float* params) {
  this_ptr->glTexParameterfv(target, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glTexParameteri(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, GLint param) {
  this_ptr->glTexParameteri(target, pname, param);
}

void qt_gui_c_QOpenGLFunctions_glTexParameteriv(QOpenGLFunctions* this_ptr, unsigned int target, unsigned int pname, const GLint* params) {
  this_ptr->glTexParameteriv(target, pname, params);
}

void qt_gui_c_QOpenGLFunctions_glTexSubImage2D(QOpenGLFunctions* this_ptr, unsigned int target, GLint level, GLint xoffset, GLint yoffset, int width, int height, unsigned int format, unsigned int type, const void* pixels) {
  this_ptr->glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
}

void qt_gui_c_QOpenGLFunctions_glUniform1f(QOpenGLFunctions* this_ptr, GLint location, float x) {
  this_ptr->glUniform1f(location, x);
}

void qt_gui_c_QOpenGLFunctions_glUniform1fv(QOpenGLFunctions* this_ptr, GLint location, int count, const float* v) {
  this_ptr->glUniform1fv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniform1i(QOpenGLFunctions* this_ptr, GLint location, GLint x) {
  this_ptr->glUniform1i(location, x);
}

void qt_gui_c_QOpenGLFunctions_glUniform1iv(QOpenGLFunctions* this_ptr, GLint location, int count, const GLint* v) {
  this_ptr->glUniform1iv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniform2f(QOpenGLFunctions* this_ptr, GLint location, float x, float y) {
  this_ptr->glUniform2f(location, x, y);
}

void qt_gui_c_QOpenGLFunctions_glUniform2fv(QOpenGLFunctions* this_ptr, GLint location, int count, const float* v) {
  this_ptr->glUniform2fv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniform2i(QOpenGLFunctions* this_ptr, GLint location, GLint x, GLint y) {
  this_ptr->glUniform2i(location, x, y);
}

void qt_gui_c_QOpenGLFunctions_glUniform2iv(QOpenGLFunctions* this_ptr, GLint location, int count, const GLint* v) {
  this_ptr->glUniform2iv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniform3f(QOpenGLFunctions* this_ptr, GLint location, float x, float y, float z) {
  this_ptr->glUniform3f(location, x, y, z);
}

void qt_gui_c_QOpenGLFunctions_glUniform3fv(QOpenGLFunctions* this_ptr, GLint location, int count, const float* v) {
  this_ptr->glUniform3fv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniform3i(QOpenGLFunctions* this_ptr, GLint location, GLint x, GLint y, GLint z) {
  this_ptr->glUniform3i(location, x, y, z);
}

void qt_gui_c_QOpenGLFunctions_glUniform3iv(QOpenGLFunctions* this_ptr, GLint location, int count, const GLint* v) {
  this_ptr->glUniform3iv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniform4f(QOpenGLFunctions* this_ptr, GLint location, float x, float y, float z, float w) {
  this_ptr->glUniform4f(location, x, y, z, w);
}

void qt_gui_c_QOpenGLFunctions_glUniform4fv(QOpenGLFunctions* this_ptr, GLint location, int count, const float* v) {
  this_ptr->glUniform4fv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniform4i(QOpenGLFunctions* this_ptr, GLint location, GLint x, GLint y, GLint z, GLint w) {
  this_ptr->glUniform4i(location, x, y, z, w);
}

void qt_gui_c_QOpenGLFunctions_glUniform4iv(QOpenGLFunctions* this_ptr, GLint location, int count, const GLint* v) {
  this_ptr->glUniform4iv(location, count, v);
}

void qt_gui_c_QOpenGLFunctions_glUniformMatrix2fv(QOpenGLFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix2fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLFunctions_glUniformMatrix3fv(QOpenGLFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix3fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLFunctions_glUniformMatrix4fv(QOpenGLFunctions* this_ptr, GLint location, int count, unsigned char transpose, const float* value) {
  this_ptr->glUniformMatrix4fv(location, count, transpose, value);
}

void qt_gui_c_QOpenGLFunctions_glUseProgram(QOpenGLFunctions* this_ptr, GLuint program) {
  this_ptr->glUseProgram(program);
}

void qt_gui_c_QOpenGLFunctions_glValidateProgram(QOpenGLFunctions* this_ptr, GLuint program) {
  this_ptr->glValidateProgram(program);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib1f(QOpenGLFunctions* this_ptr, GLuint indx, float x) {
  this_ptr->glVertexAttrib1f(indx, x);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib1fv(QOpenGLFunctions* this_ptr, GLuint indx, const float* values) {
  this_ptr->glVertexAttrib1fv(indx, values);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib2f(QOpenGLFunctions* this_ptr, GLuint indx, float x, float y) {
  this_ptr->glVertexAttrib2f(indx, x, y);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib2fv(QOpenGLFunctions* this_ptr, GLuint indx, const float* values) {
  this_ptr->glVertexAttrib2fv(indx, values);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib3f(QOpenGLFunctions* this_ptr, GLuint indx, float x, float y, float z) {
  this_ptr->glVertexAttrib3f(indx, x, y, z);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib3fv(QOpenGLFunctions* this_ptr, GLuint indx, const float* values) {
  this_ptr->glVertexAttrib3fv(indx, values);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib4f(QOpenGLFunctions* this_ptr, GLuint indx, float x, float y, float z, float w) {
  this_ptr->glVertexAttrib4f(indx, x, y, z, w);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttrib4fv(QOpenGLFunctions* this_ptr, GLuint indx, const float* values) {
  this_ptr->glVertexAttrib4fv(indx, values);
}

void qt_gui_c_QOpenGLFunctions_glVertexAttribPointer(QOpenGLFunctions* this_ptr, GLuint indx, GLint size, unsigned int type, unsigned char normalized, int stride, const void* ptr) {
  this_ptr->glVertexAttribPointer(indx, size, type, normalized, stride, ptr);
}

void qt_gui_c_QOpenGLFunctions_glViewport(QOpenGLFunctions* this_ptr, GLint x, GLint y, int width, int height) {
  this_ptr->glViewport(x, y, width, height);
}

bool qt_gui_c_QOpenGLFunctions_hasOpenGLFeature(const QOpenGLFunctions* this_ptr, const QOpenGLFunctions::OpenGLFeature* feature) {
  return this_ptr->hasOpenGLFeature(*feature);
}

void qt_gui_c_QOpenGLFunctions_initializeOpenGLFunctions(QOpenGLFunctions* this_ptr) {
  this_ptr->initializeOpenGLFunctions();
}

QOpenGLFunctions* qt_gui_c_QOpenGLFunctions_new_context(QOpenGLContext* context) {
  return new QOpenGLFunctions(context);
}

QOpenGLFunctions* qt_gui_c_QOpenGLFunctions_new_no_args() {
  return new QOpenGLFunctions();
}

