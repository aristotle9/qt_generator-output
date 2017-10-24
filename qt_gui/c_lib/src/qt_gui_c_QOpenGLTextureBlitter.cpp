#include "qt_gui_c_QOpenGLTextureBlitter.h"

void qt_gui_c_QOpenGLTextureBlitter_bind_no_args(QOpenGLTextureBlitter* this_ptr) {
  this_ptr->bind();
}

void qt_gui_c_QOpenGLTextureBlitter_bind_target(QOpenGLTextureBlitter* this_ptr, unsigned int target) {
  this_ptr->bind(target);
}

void qt_gui_c_QOpenGLTextureBlitter_blit(QOpenGLTextureBlitter* this_ptr, GLuint texture, const QMatrix4x4* targetTransform, QOpenGLTextureBlitter::Origin sourceOrigin) {
  this_ptr->blit(texture, *targetTransform, sourceOrigin);
}

void qt_gui_c_QOpenGLTextureBlitter_constructor(QOpenGLTextureBlitter* output) {
  new(output) QOpenGLTextureBlitter();
}

bool qt_gui_c_QOpenGLTextureBlitter_create(QOpenGLTextureBlitter* this_ptr) {
  return this_ptr->create();
}

void qt_gui_c_QOpenGLTextureBlitter_destroy(QOpenGLTextureBlitter* this_ptr) {
  this_ptr->destroy();
}

void qt_gui_c_QOpenGLTextureBlitter_destructor(QOpenGLTextureBlitter* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QOpenGLTextureBlitter_isCreated(const QOpenGLTextureBlitter* this_ptr) {
  return this_ptr->isCreated();
}

void qt_gui_c_QOpenGLTextureBlitter_release(QOpenGLTextureBlitter* this_ptr) {
  this_ptr->release();
}

void qt_gui_c_QOpenGLTextureBlitter_setOpacity(QOpenGLTextureBlitter* this_ptr, float opacity) {
  this_ptr->setOpacity(opacity);
}

void qt_gui_c_QOpenGLTextureBlitter_setRedBlueSwizzle(QOpenGLTextureBlitter* this_ptr, bool swizzle) {
  this_ptr->setRedBlueSwizzle(swizzle);
}

bool qt_gui_c_QOpenGLTextureBlitter_supportsExternalOESTarget(const QOpenGLTextureBlitter* this_ptr) {
  return this_ptr->supportsExternalOESTarget();
}

QMatrix4x4* qt_gui_c_QOpenGLTextureBlitter_targetTransform_as_ptr(const QRectF* target, const QRect* viewport) {
  return new QMatrix4x4(QOpenGLTextureBlitter::targetTransform(*target, *viewport));
}

