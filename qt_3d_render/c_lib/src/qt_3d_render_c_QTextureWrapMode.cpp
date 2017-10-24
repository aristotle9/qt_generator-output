#include "qt_3d_render_c_QTextureWrapMode.h"

QObject* qt_3d_render_c_QTextureWrapMode_G_static_cast_QObject_ptr(Qt3DRender::QTextureWrapMode* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DRender::QTextureWrapMode* qt_3d_render_c_QTextureWrapMode_G_static_cast_Qt3DRender_QTextureWrapMode_ptr(QObject* ptr) {
  return static_cast<Qt3DRender::QTextureWrapMode*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QTextureWrapMode_delete(Qt3DRender::QTextureWrapMode* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QTextureWrapMode_metaObject(const Qt3DRender::QTextureWrapMode* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QTextureWrapMode* qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_no_args() {
  return new Qt3DRender::QTextureWrapMode();
}

Qt3DRender::QTextureWrapMode* qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_wrapMode(Qt3DRender::QTextureWrapMode::WrapMode wrapMode) {
  return new Qt3DRender::QTextureWrapMode(wrapMode);
}

Qt3DRender::QTextureWrapMode* qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_wrapMode_parent(Qt3DRender::QTextureWrapMode::WrapMode wrapMode, QObject* parent) {
  return new Qt3DRender::QTextureWrapMode(wrapMode, parent);
}

Qt3DRender::QTextureWrapMode* qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_x_y_z(Qt3DRender::QTextureWrapMode::WrapMode x, Qt3DRender::QTextureWrapMode::WrapMode y, Qt3DRender::QTextureWrapMode::WrapMode z) {
  return new Qt3DRender::QTextureWrapMode(x, y, z);
}

Qt3DRender::QTextureWrapMode* qt_3d_render_c_Qt3DRender_QTextureWrapMode_new_x_y_z_parent(Qt3DRender::QTextureWrapMode::WrapMode x, Qt3DRender::QTextureWrapMode::WrapMode y, Qt3DRender::QTextureWrapMode::WrapMode z, QObject* parent) {
  return new Qt3DRender::QTextureWrapMode(x, y, z, parent);
}

int qt_3d_render_c_Qt3DRender_QTextureWrapMode_qt_metacall(Qt3DRender::QTextureWrapMode* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QTextureWrapMode_qt_metacast(Qt3DRender::QTextureWrapMode* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QTextureWrapMode_setX(Qt3DRender::QTextureWrapMode* this_ptr, Qt3DRender::QTextureWrapMode::WrapMode x) {
  this_ptr->setX(x);
}

void qt_3d_render_c_Qt3DRender_QTextureWrapMode_setY(Qt3DRender::QTextureWrapMode* this_ptr, Qt3DRender::QTextureWrapMode::WrapMode y) {
  this_ptr->setY(y);
}

void qt_3d_render_c_Qt3DRender_QTextureWrapMode_setZ(Qt3DRender::QTextureWrapMode* this_ptr, Qt3DRender::QTextureWrapMode::WrapMode z) {
  this_ptr->setZ(z);
}

void qt_3d_render_c_Qt3DRender_QTextureWrapMode_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTextureWrapMode::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QTextureWrapMode_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QTextureWrapMode::tr(s, c, n));
}

Qt3DRender::QTextureWrapMode::WrapMode qt_3d_render_c_Qt3DRender_QTextureWrapMode_x(const Qt3DRender::QTextureWrapMode* this_ptr) {
  return this_ptr->x();
}

Qt3DRender::QTextureWrapMode::WrapMode qt_3d_render_c_Qt3DRender_QTextureWrapMode_y(const Qt3DRender::QTextureWrapMode* this_ptr) {
  return this_ptr->y();
}

Qt3DRender::QTextureWrapMode::WrapMode qt_3d_render_c_Qt3DRender_QTextureWrapMode_z(const Qt3DRender::QTextureWrapMode* this_ptr) {
  return this_ptr->z();
}

