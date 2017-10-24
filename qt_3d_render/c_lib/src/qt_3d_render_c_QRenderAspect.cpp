#include "qt_3d_render_c_QRenderAspect.h"

QObject* qt_3d_render_c_QRenderAspect_G_static_cast_QObject_ptr(Qt3DRender::QRenderAspect* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QAbstractAspect* qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DCore_QAbstractAspect_ptr(Qt3DRender::QRenderAspect* ptr) {
  return static_cast<Qt3DCore::QAbstractAspect*>(ptr);
}

Qt3DRender::QRenderAspect* qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DRender_QRenderAspect_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QRenderAspect*>(ptr);
}

Qt3DRender::QRenderAspect* qt_3d_render_c_QRenderAspect_G_static_cast_Qt3DRender_QRenderAspect_ptr_Qt3DCore_QAbstractAspect(Qt3DCore::QAbstractAspect* ptr) {
  return static_cast<Qt3DRender::QRenderAspect*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QRenderAspect_delete(Qt3DRender::QRenderAspect* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QRenderAspect_metaObject(const Qt3DRender::QRenderAspect* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QRenderAspect* qt_3d_render_c_Qt3DRender_QRenderAspect_new_no_args() {
  return new Qt3DRender::QRenderAspect();
}

Qt3DRender::QRenderAspect* qt_3d_render_c_Qt3DRender_QRenderAspect_new_parent(QObject* parent) {
  return new Qt3DRender::QRenderAspect(parent);
}

Qt3DRender::QRenderAspect* qt_3d_render_c_Qt3DRender_QRenderAspect_new_type(Qt3DRender::QRenderAspect::RenderType type) {
  return new Qt3DRender::QRenderAspect(type);
}

Qt3DRender::QRenderAspect* qt_3d_render_c_Qt3DRender_QRenderAspect_new_type_parent(Qt3DRender::QRenderAspect::RenderType type, QObject* parent) {
  return new Qt3DRender::QRenderAspect(type, parent);
}

int qt_3d_render_c_Qt3DRender_QRenderAspect_qt_metacall(Qt3DRender::QRenderAspect* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QRenderAspect_qt_metacast(Qt3DRender::QRenderAspect* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QRenderAspect_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderAspect::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QRenderAspect_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QRenderAspect::tr(s, c, n));
}

