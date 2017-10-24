#include "qt_gui_c_QOpenGLFramebufferObjectFormat.h"

void qt_gui_c_QOpenGLFramebufferObjectFormat_constructor_no_args(QOpenGLFramebufferObjectFormat* output) {
  new(output) QOpenGLFramebufferObjectFormat();
}

void qt_gui_c_QOpenGLFramebufferObjectFormat_constructor_other(const QOpenGLFramebufferObjectFormat* other, QOpenGLFramebufferObjectFormat* output) {
  new(output) QOpenGLFramebufferObjectFormat(*other);
}

void qt_gui_c_QOpenGLFramebufferObjectFormat_destructor(QOpenGLFramebufferObjectFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

unsigned int qt_gui_c_QOpenGLFramebufferObjectFormat_internalTextureFormat(const QOpenGLFramebufferObjectFormat* this_ptr) {
  return this_ptr->internalTextureFormat();
}

bool qt_gui_c_QOpenGLFramebufferObjectFormat_mipmap(const QOpenGLFramebufferObjectFormat* this_ptr) {
  return this_ptr->mipmap();
}

QOpenGLFramebufferObjectFormat* qt_gui_c_QOpenGLFramebufferObjectFormat_operator_assign(QOpenGLFramebufferObjectFormat* this_ptr, const QOpenGLFramebufferObjectFormat* other) {
  return &this_ptr->operator=(*other);
}

bool qt_gui_c_QOpenGLFramebufferObjectFormat_operator_eq(const QOpenGLFramebufferObjectFormat* this_ptr, const QOpenGLFramebufferObjectFormat* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QOpenGLFramebufferObjectFormat_operator_neq(const QOpenGLFramebufferObjectFormat* this_ptr, const QOpenGLFramebufferObjectFormat* other) {
  return this_ptr->operator!=(*other);
}

int qt_gui_c_QOpenGLFramebufferObjectFormat_samples(const QOpenGLFramebufferObjectFormat* this_ptr) {
  return this_ptr->samples();
}

void qt_gui_c_QOpenGLFramebufferObjectFormat_setAttachment(QOpenGLFramebufferObjectFormat* this_ptr, const QOpenGLFramebufferObject::Attachment* attachment) {
  this_ptr->setAttachment(*attachment);
}

void qt_gui_c_QOpenGLFramebufferObjectFormat_setInternalTextureFormat(QOpenGLFramebufferObjectFormat* this_ptr, unsigned int internalTextureFormat) {
  this_ptr->setInternalTextureFormat(internalTextureFormat);
}

void qt_gui_c_QOpenGLFramebufferObjectFormat_setMipmap(QOpenGLFramebufferObjectFormat* this_ptr, bool enabled) {
  this_ptr->setMipmap(enabled);
}

void qt_gui_c_QOpenGLFramebufferObjectFormat_setSamples(QOpenGLFramebufferObjectFormat* this_ptr, int samples) {
  this_ptr->setSamples(samples);
}

void qt_gui_c_QOpenGLFramebufferObjectFormat_setTextureTarget(QOpenGLFramebufferObjectFormat* this_ptr, unsigned int target) {
  this_ptr->setTextureTarget(target);
}

unsigned int qt_gui_c_QOpenGLFramebufferObjectFormat_textureTarget(const QOpenGLFramebufferObjectFormat* this_ptr) {
  return this_ptr->textureTarget();
}

