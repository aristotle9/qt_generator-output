#include "qt_3d_render_c_QClipPlane.h"

Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_dynamic_cast_Qt3DRender_QClipPlane_ptr(Qt3DRender::QRenderState* ptr) {
  return dynamic_cast<Qt3DRender::QClipPlane*>(ptr);
}

QObject* qt_3d_render_c_QClipPlane_G_static_cast_QObject_ptr(Qt3DRender::QClipPlane* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QClipPlane* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QClipPlane*>(ptr);
}

Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QClipPlane*>(ptr);
}

Qt3DRender::QClipPlane* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QClipPlane_ptr_Qt3DRender_QRenderState(Qt3DRender::QRenderState* ptr) {
  return static_cast<Qt3DRender::QClipPlane*>(ptr);
}

Qt3DRender::QRenderState* qt_3d_render_c_QClipPlane_G_static_cast_Qt3DRender_QRenderState_ptr(Qt3DRender::QClipPlane* ptr) {
  return static_cast<Qt3DRender::QRenderState*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QClipPlane_delete(Qt3DRender::QClipPlane* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QClipPlane_distance(const Qt3DRender::QClipPlane* this_ptr) {
  return this_ptr->distance();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QClipPlane_metaObject(const Qt3DRender::QClipPlane* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QClipPlane* qt_3d_render_c_Qt3DRender_QClipPlane_new_no_args() {
  return new Qt3DRender::QClipPlane();
}

Qt3DRender::QClipPlane* qt_3d_render_c_Qt3DRender_QClipPlane_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QClipPlane(parent);
}

QVector3D* qt_3d_render_c_Qt3DRender_QClipPlane_normal_as_ptr(const Qt3DRender::QClipPlane* this_ptr) {
  return new QVector3D(this_ptr->normal());
}

int qt_3d_render_c_Qt3DRender_QClipPlane_planeIndex(const Qt3DRender::QClipPlane* this_ptr) {
  return this_ptr->planeIndex();
}

int qt_3d_render_c_Qt3DRender_QClipPlane_qt_metacall(Qt3DRender::QClipPlane* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QClipPlane_qt_metacast(Qt3DRender::QClipPlane* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QClipPlane_setDistance(Qt3DRender::QClipPlane* this_ptr, float arg1) {
  this_ptr->setDistance(arg1);
}

void qt_3d_render_c_Qt3DRender_QClipPlane_setNormal(Qt3DRender::QClipPlane* this_ptr, const QVector3D* arg1) {
  this_ptr->setNormal(*arg1);
}

void qt_3d_render_c_Qt3DRender_QClipPlane_setPlaneIndex(Qt3DRender::QClipPlane* this_ptr, int arg1) {
  this_ptr->setPlaneIndex(arg1);
}

void qt_3d_render_c_Qt3DRender_QClipPlane_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QClipPlane::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QClipPlane_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QClipPlane::tr(s, c, n));
}

