#include "qt_3d_extras_c_QOrbitCameraController.h"

QObject* qt_3d_extras_c_QOrbitCameraController_G_static_cast_QObject_ptr(Qt3DExtras::QOrbitCameraController* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QEntity* qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QEntity_ptr(Qt3DExtras::QOrbitCameraController* ptr) {
  return static_cast<Qt3DCore::QEntity*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QOrbitCameraController* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QOrbitCameraController* qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QOrbitCameraController*>(ptr);
}

Qt3DExtras::QOrbitCameraController* qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_Qt3DCore_QEntity(Qt3DCore::QEntity* ptr) {
  return static_cast<Qt3DExtras::QOrbitCameraController*>(ptr);
}

Qt3DExtras::QOrbitCameraController* qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QOrbitCameraController*>(ptr);
}

Qt3DRender::QCamera* qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_camera(const Qt3DExtras::QOrbitCameraController* this_ptr) {
  return this_ptr->camera();
}

void qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_delete(Qt3DExtras::QOrbitCameraController* this_ptr) {
  delete this_ptr;
}

float qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_linearSpeed(const Qt3DExtras::QOrbitCameraController* this_ptr) {
  return this_ptr->linearSpeed();
}

float qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_lookSpeed(const Qt3DExtras::QOrbitCameraController* this_ptr) {
  return this_ptr->lookSpeed();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_metaObject(const Qt3DExtras::QOrbitCameraController* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QOrbitCameraController* qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_new_no_args() {
  return new Qt3DExtras::QOrbitCameraController();
}

Qt3DExtras::QOrbitCameraController* qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QOrbitCameraController(parent);
}

int qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_qt_metacall(Qt3DExtras::QOrbitCameraController* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_qt_metacast(Qt3DExtras::QOrbitCameraController* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setCamera(Qt3DExtras::QOrbitCameraController* this_ptr, Qt3DRender::QCamera* camera) {
  this_ptr->setCamera(camera);
}

void qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setLinearSpeed(Qt3DExtras::QOrbitCameraController* this_ptr, float linearSpeed) {
  this_ptr->setLinearSpeed(linearSpeed);
}

void qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setLookSpeed(Qt3DExtras::QOrbitCameraController* this_ptr, float lookSpeed) {
  this_ptr->setLookSpeed(lookSpeed);
}

void qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setZoomInLimit(Qt3DExtras::QOrbitCameraController* this_ptr, float zoomInLimit) {
  this_ptr->setZoomInLimit(zoomInLimit);
}

void qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QOrbitCameraController::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QOrbitCameraController::tr(s, c, n));
}

float qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_zoomInLimit(const Qt3DExtras::QOrbitCameraController* this_ptr) {
  return this_ptr->zoomInLimit();
}

