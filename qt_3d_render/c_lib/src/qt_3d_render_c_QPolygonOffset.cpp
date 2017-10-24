#include "qt_3d_render_c_QPolygonOffset.h"

Qt3DRender::QPolygonOffset* qt_3d_render_c_QPolygonOffset_G_dynamic_cast_Qt3DRender_QPolygonOffset_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QPolygonOffset*>(ptr);
}

QObject* qt_3d_render_c_QPolygonOffset_G_static_cast_QObject_ptr(Qt3DRender::QPolygonOffset* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QPolygonOffset* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QPolygonOffset* qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QPolygonOffset*>(ptr);
}

Qt3DRender::QPolygonOffset* qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QPolygonOffset*>(ptr);
}

Qt3DRender::QPolygonOffset* qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QPolygonOffset_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QPolygonOffset*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QPolygonOffset_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QPolygonOffset* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QPolygonOffset_delete(Qt3DRender::QPolygonOffset* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QPolygonOffset_depthSteps(const Qt3DRender::QPolygonOffset* this_ptr) {
  return this_ptr->depthSteps();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QPolygonOffset_metaObject(const Qt3DRender::QPolygonOffset* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QPolygonOffset* qt_3d_render_c_Qt3DRender_QPolygonOffset_new_no_args() {
  return new Qt3DRender::QPolygonOffset();
}

Qt3DRender::QPolygonOffset* qt_3d_render_c_Qt3DRender_QPolygonOffset_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QPolygonOffset(parent);
}

int qt_3d_render_c_Qt3DRender_QPolygonOffset_qt_metacall(Qt3DRender::QPolygonOffset* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QPolygonOffset_qt_metacast(Qt3DRender::QPolygonOffset* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_render_c_Qt3DRender_QPolygonOffset_scaleFactor(const Qt3DRender::QPolygonOffset* this_ptr) {
  return this_ptr->scaleFactor();
}

void qt_3d_render_c_Qt3DRender_QPolygonOffset_setDepthSteps(Qt3DRender::QPolygonOffset* this_ptr, float depthSteps) {
  this_ptr->setDepthSteps(depthSteps);
}

void qt_3d_render_c_Qt3DRender_QPolygonOffset_setScaleFactor(Qt3DRender::QPolygonOffset* this_ptr, float scaleFactor) {
  this_ptr->setScaleFactor(scaleFactor);
}

void qt_3d_render_c_Qt3DRender_QPolygonOffset_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPolygonOffset::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPolygonOffset_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPolygonOffset::tr(s, c, n));
}

