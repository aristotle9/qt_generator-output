#include "qt_3d_render_c_QPointLight.h"

Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_dynamic_cast_Qt3DRender_QPointLight_ptr(Qt3DRender::QAbstractLight* ptr) {
  return dynamic_cast<Qt3DRender::QPointLight*>(ptr);
}

QObject* qt_3d_render_c_QPointLight_G_static_cast_QObject_ptr(Qt3DRender::QPointLight* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QPointLight* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QPointLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QPointLight* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QAbstractLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QAbstractLight_ptr(Qt3DRender::QPointLight* ptr) {
  return static_cast<Qt3DRender::QAbstractLight*>(ptr);
}

Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QPointLight*>(ptr);
}

Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QPointLight*>(ptr);
}

Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QPointLight*>(ptr);
}

Qt3DRender::QPointLight* qt_3d_render_c_QPointLight_G_static_cast_Qt3DRender_QPointLight_ptr_Qt3DRender_QAbstractLight(Qt3DRender::QAbstractLight* ptr) {
  return static_cast<Qt3DRender::QPointLight*>(ptr);
}

float qt_3d_render_c_Qt3DRender_QPointLight_constantAttenuation(const Qt3DRender::QPointLight* this_ptr) {
  return this_ptr->constantAttenuation();
}

void qt_3d_render_c_Qt3DRender_QPointLight_delete(Qt3DRender::QPointLight* this_ptr) {
  delete this_ptr;
}

float qt_3d_render_c_Qt3DRender_QPointLight_linearAttenuation(const Qt3DRender::QPointLight* this_ptr) {
  return this_ptr->linearAttenuation();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QPointLight_metaObject(const Qt3DRender::QPointLight* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QPointLight* qt_3d_render_c_Qt3DRender_QPointLight_new_no_args() {
  return new Qt3DRender::QPointLight();
}

Qt3DRender::QPointLight* qt_3d_render_c_Qt3DRender_QPointLight_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QPointLight(parent);
}

int qt_3d_render_c_Qt3DRender_QPointLight_qt_metacall(Qt3DRender::QPointLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QPointLight_qt_metacast(Qt3DRender::QPointLight* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

float qt_3d_render_c_Qt3DRender_QPointLight_quadraticAttenuation(const Qt3DRender::QPointLight* this_ptr) {
  return this_ptr->quadraticAttenuation();
}

void qt_3d_render_c_Qt3DRender_QPointLight_setConstantAttenuation(Qt3DRender::QPointLight* this_ptr, float value) {
  this_ptr->setConstantAttenuation(value);
}

void qt_3d_render_c_Qt3DRender_QPointLight_setLinearAttenuation(Qt3DRender::QPointLight* this_ptr, float value) {
  this_ptr->setLinearAttenuation(value);
}

void qt_3d_render_c_Qt3DRender_QPointLight_setQuadraticAttenuation(Qt3DRender::QPointLight* this_ptr, float value) {
  this_ptr->setQuadraticAttenuation(value);
}

void qt_3d_render_c_Qt3DRender_QPointLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPointLight::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QPointLight_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QPointLight::tr(s, c, n));
}

