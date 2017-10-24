#include "qt_gui_c_QOpenGLBuffer.h"

void qt_gui_c_QOpenGLBuffer_allocate_count(QOpenGLBuffer* this_ptr, int count) {
  this_ptr->allocate(count);
}

void qt_gui_c_QOpenGLBuffer_allocate_data_count(QOpenGLBuffer* this_ptr, const void* data, int count) {
  this_ptr->allocate(data, count);
}

bool qt_gui_c_QOpenGLBuffer_bind(QOpenGLBuffer* this_ptr) {
  return this_ptr->bind();
}

GLuint qt_gui_c_QOpenGLBuffer_bufferId(const QOpenGLBuffer* this_ptr) {
  return this_ptr->bufferId();
}

void qt_gui_c_QOpenGLBuffer_constructor_no_args(QOpenGLBuffer* output) {
  new(output) QOpenGLBuffer();
}

void qt_gui_c_QOpenGLBuffer_constructor_other(const QOpenGLBuffer* other, QOpenGLBuffer* output) {
  new(output) QOpenGLBuffer(*other);
}

void qt_gui_c_QOpenGLBuffer_constructor_type(const QOpenGLBuffer::Type* type, QOpenGLBuffer* output) {
  new(output) QOpenGLBuffer(*type);
}

bool qt_gui_c_QOpenGLBuffer_create(QOpenGLBuffer* this_ptr) {
  return this_ptr->create();
}

void qt_gui_c_QOpenGLBuffer_destroy(QOpenGLBuffer* this_ptr) {
  this_ptr->destroy();
}

void qt_gui_c_QOpenGLBuffer_destructor(QOpenGLBuffer* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QOpenGLBuffer_isCreated(const QOpenGLBuffer* this_ptr) {
  return this_ptr->isCreated();
}

void* qt_gui_c_QOpenGLBuffer_map(QOpenGLBuffer* this_ptr, const QOpenGLBuffer::Access* access) {
  return this_ptr->map(*access);
}

QOpenGLBuffer* qt_gui_c_QOpenGLBuffer_operator_assign(QOpenGLBuffer* this_ptr, const QOpenGLBuffer* other) {
  return &this_ptr->operator=(*other);
}

bool qt_gui_c_QOpenGLBuffer_read(QOpenGLBuffer* this_ptr, int offset, void* data, int count) {
  return this_ptr->read(offset, data, count);
}

void qt_gui_c_QOpenGLBuffer_release_no_args(QOpenGLBuffer* this_ptr) {
  this_ptr->release();
}

void qt_gui_c_QOpenGLBuffer_release_type(const QOpenGLBuffer::Type* type) {
  QOpenGLBuffer::release(*type);
}

void qt_gui_c_QOpenGLBuffer_setUsagePattern(QOpenGLBuffer* this_ptr, const QOpenGLBuffer::UsagePattern* value) {
  this_ptr->setUsagePattern(*value);
}

int qt_gui_c_QOpenGLBuffer_size(const QOpenGLBuffer* this_ptr) {
  return this_ptr->size();
}

bool qt_gui_c_QOpenGLBuffer_unmap(QOpenGLBuffer* this_ptr) {
  return this_ptr->unmap();
}

void qt_gui_c_QOpenGLBuffer_write(QOpenGLBuffer* this_ptr, int offset, const void* data, int count) {
  this_ptr->write(offset, data, count);
}

