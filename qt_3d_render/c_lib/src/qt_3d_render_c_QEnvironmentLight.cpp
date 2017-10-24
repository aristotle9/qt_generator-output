#include "qt_3d_render_c_QEnvironmentLight.h"

QObject* qt_3d_render_c_QEnvironmentLight_G_static_cast_QObject_ptr(Qt3DRender::QEnvironmentLight* ptr) {
  return static_cast<QObject*>(ptr);
}

Qt3DCore::QComponent* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QComponent_ptr(Qt3DRender::QEnvironmentLight* ptr) {
  return static_cast<Qt3DCore::QComponent*>(ptr);
}

Qt3DCore::QNode* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DCore_QNode_ptr(Qt3DRender::QEnvironmentLight* ptr) {
  return static_cast<Qt3DCore::QNode*>(ptr);
}

Qt3DRender::QEnvironmentLight* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_QObject(QObject* ptr) {
  return static_cast<Qt3DRender::QEnvironmentLight*>(ptr);
}

Qt3DRender::QEnvironmentLight* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QComponent(Qt3DCore::QComponent* ptr) {
  return static_cast<Qt3DRender::QEnvironmentLight*>(ptr);
}

Qt3DRender::QEnvironmentLight* qt_3d_render_c_QEnvironmentLight_G_static_cast_Qt3DRender_QEnvironmentLight_ptr_Qt3DCore_QNode(Qt3DCore::QNode* ptr) {
  return static_cast<Qt3DRender::QEnvironmentLight*>(ptr);
}

void qt_3d_render_c_Qt3DRender_QEnvironmentLight_delete(Qt3DRender::QEnvironmentLight* this_ptr) {
  delete this_ptr;
}

Qt3DRender::QAbstractTexture* qt_3d_render_c_Qt3DRender_QEnvironmentLight_irradiance(const Qt3DRender::QEnvironmentLight* this_ptr) {
  return this_ptr->irradiance();
}

const QMetaObject* qt_3d_render_c_Qt3DRender_QEnvironmentLight_metaObject(const Qt3DRender::QEnvironmentLight* this_ptr) {
  return this_ptr->metaObject();
}

Qt3DRender::QEnvironmentLight* qt_3d_render_c_Qt3DRender_QEnvironmentLight_new_no_args() {
  return new Qt3DRender::QEnvironmentLight();
}

Qt3DRender::QEnvironmentLight* qt_3d_render_c_Qt3DRender_QEnvironmentLight_new_parent(Qt3DCore::QNode* parent) {
  return new Qt3DRender::QEnvironmentLight(parent);
}

int qt_3d_render_c_Qt3DRender_QEnvironmentLight_qt_metacall(Qt3DRender::QEnvironmentLight* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_3d_render_c_Qt3DRender_QEnvironmentLight_qt_metacast(Qt3DRender::QEnvironmentLight* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_3d_render_c_Qt3DRender_QEnvironmentLight_setIrradiance(Qt3DRender::QEnvironmentLight* this_ptr, Qt3DRender::QAbstractTexture* irradiance) {
  this_ptr->setIrradiance(irradiance);
}

void qt_3d_render_c_Qt3DRender_QEnvironmentLight_setSpecular(Qt3DRender::QEnvironmentLight* this_ptr, Qt3DRender::QAbstractTexture* specular) {
  this_ptr->setSpecular(specular);
}

Qt3DRender::QAbstractTexture* qt_3d_render_c_Qt3DRender_QEnvironmentLight_specular(const Qt3DRender::QEnvironmentLight* this_ptr) {
  return this_ptr->specular();
}

void qt_3d_render_c_Qt3DRender_QEnvironmentLight_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QEnvironmentLight::trUtf8(s, c, n));
}

void qt_3d_render_c_Qt3DRender_QEnvironmentLight_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(Qt3DRender::QEnvironmentLight::tr(s, c, n));
}

