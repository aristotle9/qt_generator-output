#include "qt_3d_render_c_QCameraSelector.h"

Qt3DRender::QCameraSelector* qt_3d_render_c_QCameraSelector_G_dynamic_cast_Qt3DRender_QCameraSelector_ptr(Qt3DRender::QFrameGraphNode* ptr) {
  return dynamic_cast<Qt3DRender::QCameraSelector*>(ptr);
}

QObject* qt_3d_render_c_QCameraSelector_G_static_cast_QObject_ptr(Qt3DRender::QCameraSelector* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QCameraSelector* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QCameraSelector* qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QCameraSelector*>(ptr);
}

Qt3DRender::QCameraSelector* qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QCameraSelector*>(ptr);
}

Qt3DRender::QCameraSelector* qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QCameraSelector_ptr_Qt3DRender_QFrameGraphNode(Qt3DRender::QFrameGraphNode* ptr) {
  return static_cast<Qt3DRender::QCameraSelector*>(ptr);
}

Qt3DRender::QFrameGraphNode* qt_3d_render_c_QCameraSelector_G_static_cast_Qt3DRender_QFrameGraphNode_ptr(Qt3DRender::QCameraSelector* ptr) {
  return static_cast<Qt3DRender::QFrameGraphNode*>(ptr);
}

Qt3DCore::QEntity* qt_3d_render_c_Qt3DRender_QCameraSelector_camera(const Qt3DRender::QCameraSelector* this_ptr) {
  return this_ptr->camera();
}

void qt_3d_render_c_Qt3DRender_QCameraSelector_delete(Qt3DRender::QCameraSelector* this_ptr) {
  delete this_ptr;
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QCameraSelector_metaObject(const Qt3DRender::QCameraSelector* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QCameraSelector* qt_3d_render_c_Qt3DRender_QCameraSelector_new_no_args() {
  return new Qt3DRender::QCameraSelector();
}

Qt3DRender::QCameraSelector* qt_3d_render_c_Qt3DRender_QCameraSelector_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QCameraSelector(parent);
}

int qt_3d_render_c_Qt3DRender_QCameraSelector_qt_metacall(Qt3DRender::QCameraSelector* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QCameraSelector_qt_metacast(Qt3DRender::QCameraSelector* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QCameraSelector_setCamera(Qt3DRender::QCameraSelector* this_ptr, Qt3DCore::QEntity* camera) {
  this_ptr->setCamera(camera);
}

void qt_3d_render_c_Qt3DRender_QCameraSelector_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCameraSelector::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QCameraSelector_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QCameraSelector::tr(s, c, n));
}

