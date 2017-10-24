#include "qt_3d_render_c_QViewport.h"

Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_dynamic_cast_Qt3DRender_QViewport_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QViewport*>(ptr);
}

QObject* qt_3d_render_c_QViewport_G_static_cast_QObject_ptr(Qt3DRender::QViewport* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QViewport_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QViewport* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QViewport* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QViewport*>(ptr);
}

Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QViewport*>(ptr);
}

Qt3DRender::QViewport* qt_3d_render_c_QViewport_G_static_cast_Qt3DRender_QViewport_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QViewport*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QViewport_delete(Qt3DRender::QViewport* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QViewport_gamma(const Qt3DRender::QViewport* this_ptr) {
  return this_ptr->gamma();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QViewport_metaObject(const Qt3DRender::QViewport* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QViewport* qt_3d_render_c_Qt3DRender_QViewport_new_no_args() {
  return new Qt3DRender::QViewport();
}

Qt3DRender::QViewport* qt_3d_render_c_Qt3DRender_QViewport_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QViewport(parent);
}

void qt_3d_render_c_Qt3DRender_QViewport_normalizedRect_to_output(const Qt3DRender::QViewport* this_ptr, QRectF* output) {
  new(output) QRectF(this_ptr->normalizedRect());
}

int qt_3d_render_c_Qt3DRender_QViewport_qt_metacall(Qt3DRender::QViewport* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QViewport_qt_metacast(Qt3DRender::QViewport* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QViewport_setGamma(Qt3DRender::QViewport* this_ptr, float gamma) {
  this_ptr->setGamma(gamma);
}

void qt_3d_render_c_Qt3DRender_QViewport_setNormalizedRect(Qt3DRender::QViewport* this_ptr, const QRectF* normalizedRect) {
  this_ptr->setNormalizedRect(*normalizedRect);
}

void qt_3d_render_c_Qt3DRender_QViewport_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QViewport::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QViewport_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QViewport::tr(s, c, n));
}

