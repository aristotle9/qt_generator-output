#include "qt_3d_render_c_QDispatchCompute.h"

Qt3DRender::QDispatchCompute* qt_3d_render_c_QDispatchCompute_G_dynamic_cast_Qt3DRender_QDispatchCompute_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QDispatchCompute*>(ptr);
}

QObject* qt_3d_render_c_QDispatchCompute_G_static_cast_QObject_ptr(Qt3DRender::QDispatchCompute* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QDispatchCompute* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QDispatchCompute* qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QDispatchCompute*>(ptr);
}

Qt3DRender::QDispatchCompute* qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QDispatchCompute*>(ptr);
}

Qt3DRender::QDispatchCompute* qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QDispatchCompute_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QDispatchCompute*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QDispatchCompute_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QDispatchCompute* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QDispatchCompute_delete(Qt3DRender::QDispatchCompute* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QDispatchCompute_metaObject(const Qt3DRender::QDispatchCompute* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QDispatchCompute* qt_3d_render_c_Qt3DRender_QDispatchCompute_new_no_args() {
  return new Qt3DRender::QDispatchCompute();
}

Qt3DRender::QDispatchCompute* qt_3d_render_c_Qt3DRender_QDispatchCompute_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QDispatchCompute(parent);
}

int qt_3d_render_c_Qt3DRender_QDispatchCompute_qt_metacall(Qt3DRender::QDispatchCompute* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QDispatchCompute_qt_metacast(Qt3DRender::QDispatchCompute* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QDispatchCompute_setWorkGroupX(Qt3DRender::QDispatchCompute* this_ptr, int workGroupX) {
  this_ptr->setWorkGroupX(workGroupX);
}

void qt_3d_render_c_Qt3DRender_QDispatchCompute_setWorkGroupY(Qt3DRender::QDispatchCompute* this_ptr, int workGroupY) {
  this_ptr->setWorkGroupY(workGroupY);
}

void qt_3d_render_c_Qt3DRender_QDispatchCompute_setWorkGroupZ(Qt3DRender::QDispatchCompute* this_ptr, int workGroupZ) {
  this_ptr->setWorkGroupZ(workGroupZ);
}

void qt_3d_render_c_Qt3DRender_QDispatchCompute_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QDispatchCompute::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QDispatchCompute_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QDispatchCompute::tr(s, c, n));
}

int qt_3d_render_c_Qt3DRender_QDispatchCompute_workGroupX(const Qt3DRender::QDispatchCompute* this_ptr) {
  return this_ptr->workGroupX();
}

int qt_3d_render_c_Qt3DRender_QDispatchCompute_workGroupY(const Qt3DRender::QDispatchCompute* this_ptr) {
  return this_ptr->workGroupY();
}

int qt_3d_render_c_Qt3DRender_QDispatchCompute_workGroupZ(const Qt3DRender::QDispatchCompute* this_ptr) {
  return this_ptr->workGroupZ();
}

