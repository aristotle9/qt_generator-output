#include "qt_3d_render_c_QPickingSettings.h"

QObject* qt_3d_render_c_QPickingSettings_G_static_cast_QObject_ptr(Qt3DRender::QPickingSettings* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QPickingSettings* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QPickingSettings* qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QPickingSettings*>(ptr);
}

Qt3DRender::QPickingSettings* qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QPickingSettings*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QPickingSettings_delete(Qt3DRender::QPickingSettings* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QPickingSettings::FaceOrientationPickingMode qt_3d_render_c_Qt3DRender_QPickingSettings_faceOrientationPickingMode(const Qt3DRender::QPickingSettings* this_ptr) {
  return this_ptr->faceOrientationPickingMode();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QPickingSettings_metaObject(const Qt3DRender::QPickingSettings* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QPickingSettings* qt_3d_render_c_Qt3DRender_QPickingSettings_new_no_args() {
  return new Qt3DRender::QPickingSettings();
}

Qt3DRender::QPickingSettings* qt_3d_render_c_Qt3DRender_QPickingSettings_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QPickingSettings(parent);
}

Qt3DRender::QPickingSettings::PickMethod qt_3d_render_c_Qt3DRender_QPickingSettings_pickMethod(const Qt3DRender::QPickingSettings* this_ptr) {
  return this_ptr->pickMethod();
}

Qt3DRender::QPickingSettings::PickResultMode qt_3d_render_c_Qt3DRender_QPickingSettings_pickResultMode(const Qt3DRender::QPickingSettings* this_ptr) {
  return this_ptr->pickResultMode();
}

int qt_3d_render_c_Qt3DRender_QPickingSettings_qt_metacall(Qt3DRender::QPickingSettings* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QPickingSettings_qt_metacast(Qt3DRender::QPickingSettings* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QPickingSettings_setFaceOrientationPickingMode(Qt3DRender::QPickingSettings* this_ptr, Qt3DRender::QPickingSettings::FaceOrientationPickingMode faceOrientationPickingMode) {
  this_ptr->setFaceOrientationPickingMode(faceOrientationPickingMode);
}

void qt_3d_render_c_Qt3DRender_QPickingSettings_setPickMethod(Qt3DRender::QPickingSettings* this_ptr, Qt3DRender::QPickingSettings::PickMethod pickMethod) {
  this_ptr->setPickMethod(pickMethod);
}

void qt_3d_render_c_Qt3DRender_QPickingSettings_setPickResultMode(Qt3DRender::QPickingSettings* this_ptr, Qt3DRender::QPickingSettings::PickResultMode pickResultMode) {
  this_ptr->setPickResultMode(pickResultMode);
}

void qt_3d_render_c_Qt3DRender_QPickingSettings_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPickingSettings::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPickingSettings_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPickingSettings::tr(s, c, n));
}

