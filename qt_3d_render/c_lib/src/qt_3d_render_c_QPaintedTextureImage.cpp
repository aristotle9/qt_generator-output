#include "qt_3d_render_c_QPaintedTextureImage.h"

Qt3DRender::QPaintedTextureImage* qt_3d_render_c_QPaintedTextureImage_G_dynamic_cast_Qt3DRender_QPaintedTextureImage_ptr(Qt3DRender::QAbstractTextureImage* ptr) {
  return dynamic_cast<Qt3DRender::QPaintedTextureImage*>(ptr);
}

QObject* qt_3d_render_c_QPaintedTextureImage_G_static_cast_QObject_ptr(Qt3DRender::QPaintedTextureImage* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QPaintedTextureImage* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractTextureImage* qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(Qt3DRender::QPaintedTextureImage* ptr) {
  return static_cast<Qt3DRender::QAbstractTextureImage*>(ptr);
}

Qt3DRender::QPaintedTextureImage* qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QPaintedTextureImage*>(ptr);
}

Qt3DRender::QPaintedTextureImage* qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QPaintedTextureImage*>(ptr);
}

Qt3DRender::QPaintedTextureImage* qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_Qt3DRender_QAbstractTextureImage(Qt3DRender::QAbstractTextureImage* ptr) {
  return static_cast<Qt3DRender::QPaintedTextureImage*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_delete(Qt3DRender::QPaintedTextureImage* this_ptr) {
  delete this_ptr;
}

int qt_3d_render_c_Qt3DRender_QPaintedTextureImage_height(const Qt3DRender::QPaintedTextureImage* this_ptr) {
  return this_ptr->height();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QPaintedTextureImage_metaObject(const Qt3DRender::QPaintedTextureImage* this_ptr) {
  return this_ptr->metaObject();
}

int qt_3d_render_c_Qt3DRender_QPaintedTextureImage_qt_metacall(Qt3DRender::QPaintedTextureImage* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QPaintedTextureImage_qt_metacast(Qt3DRender::QPaintedTextureImage* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_setHeight(Qt3DRender::QPaintedTextureImage* this_ptr, int h) {
  this_ptr->setHeight(h);
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_setSize(Qt3DRender::QPaintedTextureImage* this_ptr, const QSize* size) {
  this_ptr->setSize(*size);
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_setWidth(Qt3DRender::QPaintedTextureImage* this_ptr, int w) {
  this_ptr->setWidth(w);
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_size_to_output(const Qt3DRender::QPaintedTextureImage* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->size());
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPaintedTextureImage::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPaintedTextureImage::tr(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_update_no_args(Qt3DRender::QPaintedTextureImage* this_ptr) {
  this_ptr->update();
}

void qt_3d_render_c_Qt3DRender_QPaintedTextureImage_update_rect(Qt3DRender::QPaintedTextureImage* this_ptr, const QRect* rect) {
  this_ptr->update(*rect);
}

int qt_3d_render_c_Qt3DRender_QPaintedTextureImage_width(const Qt3DRender::QPaintedTextureImage* this_ptr) {
  return this_ptr->width();
}

