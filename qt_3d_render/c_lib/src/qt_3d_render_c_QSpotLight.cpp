#include "qt_3d_render_c_QSpotLight.h"

Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_dynamic_cast_Qt3DRender_QSpotLight_ptr(Qt3DRender::QAbstractLight* ptr) {
  return dynamic_cast<Qt3DRender::QSpotLight*>(ptr);
}

QObject* qt_3d_render_c_QSpotLight_G_static_cast_QObject_ptr(Qt3DRender::QSpotLight* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QSpotLight* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QSpotLight* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(Qt3DRender::QSpotLight* ptr) {
  return static_cast<Qt3DRender::QAbstractLight*>(ptr);
}

Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QSpotLight*>(ptr);
}

Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QSpotLight*>(ptr);
}

Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QSpotLight*>(ptr);
}

Qt3DRender::QSpotLight* qt_3d_render_c_QSpotLight_G_static_cast_Qt3DRender_QSpotLight_ptr_Qt3DRender_QAbstractLight(Qt3DRender::QAbstractLight* ptr) {
  return static_cast<Qt3DRender::QSpotLight*>(ptr);
}

float qt_3d_render_c_Qt3DRender_QSpotLight_constantAttenuation(const Qt3DRender::QSpotLight* this_ptr) {
  return this_ptr->constantAttenuation();
}

float qt_3d_render_c_Qt3DRender_QSpotLight_cutOffAngle(const Qt3DRender::QSpotLight* this_ptr) {
  return this_ptr->cutOffAngle();
}

void qt_3d_render_c_Qt3DRender_QSpotLight_delete(Qt3DRender::QSpotLight* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QSpotLight_linearAttenuation(const Qt3DRender::QSpotLight* this_ptr) {
  return this_ptr->linearAttenuation();
}

QVector3D* qt_3d_render_c_Qt3DRender_QSpotLight_localDirection_as_ptr(const Qt3DRender::QSpotLight* this_ptr) {
  return new QVector3D(this_ptr->localDirection());
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QSpotLight_metaObject(const Qt3DRender::QSpotLight* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QSpotLight* qt_3d_render_c_Qt3DRender_QSpotLight_new_no_args() {
  return new Qt3DRender::QSpotLight();
}

Qt3DRender::QSpotLight* qt_3d_render_c_Qt3DRender_QSpotLight_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QSpotLight(parent);
}

int qt_3d_render_c_Qt3DRender_QSpotLight_qt_metacall(Qt3DRender::QSpotLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QSpotLight_qt_metacast(Qt3DRender::QSpotLight* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_render_c_Qt3DRender_QSpotLight_quadraticAttenuation(const Qt3DRender::QSpotLight* this_ptr) {
  return this_ptr->quadraticAttenuation();
}

void qt_3d_render_c_Qt3DRender_QSpotLight_setConstantAttenuation(Qt3DRender::QSpotLight* this_ptr, float value) {
  this_ptr->setConstantAttenuation(value);
}

void qt_3d_render_c_Qt3DRender_QSpotLight_setCutOffAngle(Qt3DRender::QSpotLight* this_ptr, float cutOffAngle) {
  this_ptr->setCutOffAngle(cutOffAngle);
}

void qt_3d_render_c_Qt3DRender_QSpotLight_setLinearAttenuation(Qt3DRender::QSpotLight* this_ptr, float value) {
  this_ptr->setLinearAttenuation(value);
}

void qt_3d_render_c_Qt3DRender_QSpotLight_setLocalDirection(Qt3DRender::QSpotLight* this_ptr, const QVector3D* localDirection) {
  this_ptr->setLocalDirection(*localDirection);
}

void qt_3d_render_c_Qt3DRender_QSpotLight_setQuadraticAttenuation(Qt3DRender::QSpotLight* this_ptr, float value) {
  this_ptr->setQuadraticAttenuation(value);
}

void qt_3d_render_c_Qt3DRender_QSpotLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QSpotLight::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QSpotLight_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QSpotLight::tr(s, c, n));
}

