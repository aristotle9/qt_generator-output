#include "qt_3d_extras_c_QFirstPersonCameraController.h"

QObject* qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_QObject_ptr(Qt3DExtras::QFirstPersonCameraController* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QEntity* qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QEntity_ptr(Qt3DExtras::QFirstPersonCameraController* ptr) {
  return static_cast<Qt3DCore::QEntity*>(ptr);
}

Qt3DCore::QNode* qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QNode_ptr(Qt3DExtras::QFirstPersonCameraController* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DExtras::QFirstPersonCameraController* qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DExtras::QFirstPersonCameraController*>(ptr);
}

Qt3DExtras::QFirstPersonCameraController* qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_Qt3DCore_QEntity(Qt3DCore::QEntity* ptr) {
  return static_cast<Qt3DExtras::QFirstPersonCameraController*>(ptr);
}

Qt3DExtras::QFirstPersonCameraController* qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DExtras::QFirstPersonCameraController*>(ptr);
}

float qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_acceleration(const Qt3DExtras::QFirstPersonCameraController* this_ptr) {
  return this_ptr->acceleration();
}

Qt3DRender::QCamera* qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_camera(const Qt3DExtras::QFirstPersonCameraController* this_ptr) {
  return this_ptr->camera();
}

float qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_deceleration(const Qt3DExtras::QFirstPersonCameraController* this_ptr) {
  return this_ptr->deceleration();
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_delete(Qt3DExtras::QFirstPersonCameraController* this_ptr) {
  delete this_ptr;
}

float qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_linearSpeed(const Qt3DExtras::QFirstPersonCameraController* this_ptr) {
  return this_ptr->linearSpeed();
}

float qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_lookSpeed(const Qt3DExtras::QFirstPersonCameraController* this_ptr) {
  return this_ptr->lookSpeed();
}

const QMetaObject* qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_metaObject(const Qt3DExtras::QFirstPersonCameraController* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DExtras::QFirstPersonCameraController* qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_new_no_args() {
  return new Qt3DExtras::QFirstPersonCameraController();
}

Qt3DExtras::QFirstPersonCameraController* qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DExtras::QFirstPersonCameraController(parent);
}

int qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_qt_metacall(Qt3DExtras::QFirstPersonCameraController* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_qt_metacast(Qt3DExtras::QFirstPersonCameraController* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setAcceleration(Qt3DExtras::QFirstPersonCameraController* this_ptr, float acceleration) {
  this_ptr->setAcceleration(acceleration);
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setCamera(Qt3DExtras::QFirstPersonCameraController* this_ptr, Qt3DRender::QCamera* camera) {
  this_ptr->setCamera(camera);
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setDeceleration(Qt3DExtras::QFirstPersonCameraController* this_ptr, float deceleration) {
  this_ptr->setDeceleration(deceleration);
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setLinearSpeed(Qt3DExtras::QFirstPersonCameraController* this_ptr, float linearSpeed) {
  this_ptr->setLinearSpeed(linearSpeed);
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setLookSpeed(Qt3DExtras::QFirstPersonCameraController* this_ptr, float lookSpeed) {
  this_ptr->setLookSpeed(lookSpeed);
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QFirstPersonCameraController::trUtf8(s, c, n));
}

void qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DExtras::QFirstPersonCameraController::tr(s, c, n));
}

